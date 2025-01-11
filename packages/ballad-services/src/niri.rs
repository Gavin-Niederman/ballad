use std::cell::LazyCell;

use gtk::glib::{self, Object};

#[derive(Debug, Default, Clone, PartialEq, Eq, glib::Boxed)]
#[boxed_type(name = "NiriOutput")]
pub struct Output {
    /// Physical connector of the output (name).
    pub connector: String,
    /// Textual description of the manufacturer.
    pub make: String,
    /// Textual description of the model.
    pub model: String,
    /// Serial of the output, if known.
    pub serial: Option<String>,
    /// Physical width and height of the output in millimeters, if known.
    pub physical_size: Option<(u32, u32)>,
    /// Whether the output supports variable refresh rate.
    pub vrr_supported: bool,
    /// Whether variable refresh rate is enabled on the output.
    pub vrr_enabled: bool,
}

#[derive(Clone, Copy, Debug, Default, glib::Boxed)]
#[boxed_type(name = "OptionalId")]
pub enum OptionalId {
    Id(u64),
    #[default]
    None,
}

impl From<Option<u64>> for OptionalId {
    fn from(id: Option<u64>) -> Self {
        match id {
            Some(id) => Self::Id(id),
            None => Self::None,
        }
    }
}
impl From<OptionalId> for Option<u64> {
    fn from(id: OptionalId) -> Self {
        match id {
            OptionalId::Id(id) => Some(id),
            OptionalId::None => None,
        }
    }
}

mod workspace_imp {
    use std::cell::{Cell, RefCell};
    use std::sync::OnceLock;

    use gtk::glib;
    use gtk::glib::subclass::Signal;
    use gtk::{glib::Properties, prelude::*, subclass::prelude::*};

    use super::OptionalId;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::Workspace)]
    pub struct Workspace {
        /// Unique id of this workspace.
        #[property(get)]
        pub(super) id: Cell<u64>,
        /// Index of the workspace on its monitor.
        ///
        /// This is the same index you can use for requests like `niri msg action focus-workspace`.
        #[property(get)]
        pub(super) idx: Cell<u8>,
        /// Optional name of the workspace.
        #[property(get)]
        pub(super) name: RefCell<Option<String>>,
        /// Name of the output that the workspace is on.
        ///
        /// Can be `None` if no outputs are currently connected.
        #[property(get)]
        pub(super) output: RefCell<Option<String>>,
        /// Whether the workspace is currently active on its output.
        #[property(get)]
        pub(super) is_active: Cell<bool>,
        /// Whether the workspace is currently focused.
        #[property(get)]
        pub(super) is_focused: Cell<bool>,
        /// Id of the active window on this workspace, if any.
        #[property(get)]
        pub(super) active_window_id: Cell<OptionalId>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Workspace {
        const NAME: &'static str = "BalladServicesNiriWorkspace";
        type Type = super::Workspace;
    }

    #[glib::derived_properties]
    impl ObjectImpl for Workspace {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| vec![Signal::builder("changed").build()])
        }
    }
}

mod window_imp {
    use std::cell::{Cell, RefCell};
    use std::sync::OnceLock;

    use gtk::glib;
    use gtk::glib::subclass::Signal;
    use gtk::{glib::Properties, prelude::*, subclass::prelude::*};

    use super::OptionalId;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::Window)]
    pub struct Window {
        /// Unique id of this window.
        #[property(get)]
        pub(super) id: Cell<u64>,
        /// Title, if set.
        #[property(get)]
        pub(super) title: RefCell<Option<String>>,
        /// Application ID, if set.
        #[property(get)]
        pub(super) app_id: RefCell<Option<String>>,
        /// Id of the workspace this window is on, if any.
        #[property(get)]
        pub(super) workspace_id: Cell<OptionalId>,
        /// Whether this window is currently focused.
        #[property(get)]
        pub(super) is_focused: Cell<bool>,
        #[property(get)]
        pub(super) is_active: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "BalladServicesNiriWindow";
        type Type = super::Window;
    }

    #[glib::derived_properties]
    impl ObjectImpl for Window {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| vec![Signal::builder("changed").build()])
        }
    }
}

mod imp {
    use std::cell::RefCell;
    use std::net::Shutdown;
    use std::sync::OnceLock;

    use futures::AsyncBufReadExt;
    use gtk::gio::ListStore;
    use gtk::glib;
    use gtk::glib::clone;
    use gtk::glib::subclass::Signal;
    use gtk::{glib::Properties, prelude::*, subclass::prelude::*};
    use smol::io::{AsyncWriteExt, BufReader};
    use smol::net::unix::UnixStream;
    use smol::stream::StreamExt;

    use super::{OptionalId, Window, Workspace};

    fn construct_window_object(ipc_window: &niri_ipc::Window, object: &Window) {
        object.imp().id.set(ipc_window.id);
        object.notify_id();
        object.imp().title.replace(ipc_window.title.clone());
        object.notify_title();
        object.imp().app_id.replace(ipc_window.app_id.clone());
        object.notify_app_id();
        object
            .imp()
            .workspace_id
            .set(ipc_window.workspace_id.into());
        object.notify_workspace_id();
        object.imp().is_focused.set(ipc_window.is_focused);
        object.notify_is_focused();

        object.emit_by_name::<()>("changed", &[]);
    }

    #[derive(Properties)]
    #[properties(wrapper_type = super::NiriService)]
    pub struct NiriService {
        // #[property(get)]
        // outputs: RefCell<ListStore>,
        #[property(get)]
        windows: RefCell<ListStore>,
        #[property(get)]
        workspaces: RefCell<ListStore>,

        #[property(get)]
        focused_window: RefCell<Option<Window>>,
        #[property(get)]
        active_windows: RefCell<ListStore>,
    }

    impl NiriService {
        pub fn update_active_windows(&self) {
            for workspace in self.workspaces.borrow().iter::<Workspace>() {
                let _ = workspace.map(|workspace| {
                    if let OptionalId::Id(id) = workspace.active_window_id() {
                        let Some(window) = self
                            .windows
                            .borrow()
                            .iter::<Window>()
                            .filter_map(|window| window.ok())
                            .find(|window| window.id() == id)
                        else {
                            dbg!("Active window not found.");
                            return;
                        };
                        window.imp().is_active.set(true);
                        window.notify_is_active();
                        window.emit_by_name::<()>("changed", &[]);
                        self.active_windows.borrow_mut().append(&window);
                    }
                });
            }
        }

        pub fn window_by_id(&self, id: u64) -> Option<Window> {
            self.windows
                .borrow()
                .iter::<Window>()
                .filter_map(|window| window.ok())
                .find(|window| window.id() == id)
        }

        pub fn handle_niri_event(&self, event: niri_ipc::Event) {
            match event {
                niri_ipc::Event::WindowsChanged { windows } => {
                    self.windows.borrow_mut().remove_all();
                    self.focused_window.replace(None);

                    for window in windows {
                        let new_window = super::Window::new();
                        construct_window_object(&window, &new_window);

                        if window.is_focused {
                            self.focused_window.replace(Some(new_window.clone()));
                            self.obj().notify_focused_window();
                        }

                        self.windows.borrow_mut().append(&new_window);
                    }

                    self.update_active_windows();

                    self.obj()
                        .emit_by_name::<()>("windows-changed", &[&*self.windows.borrow()]);
                }
                niri_ipc::Event::WindowFocusChanged { id } => {
                    if let Some(window) = self.focused_window.borrow().as_ref() {
                        window.imp().is_focused.set(false);
                        window.notify_is_focused();
                        window.emit_by_name::<()>("changed", &[]);
                    }

                    if let Some(id) = id {
                        let Some(focused_window) = self.window_by_id(id) else {
                            dbg!("Focused window not found.");
                            return;
                        };
                        focused_window.imp().is_focused.set(true);
                        focused_window.notify_is_focused();
                        focused_window.emit_by_name::<()>("changed", &[]);

                        self.focused_window.replace(Some(focused_window.clone()));
                    }
                }
                niri_ipc::Event::WindowOpenedOrChanged { window: ipc_window } => {
                    if let Some(window) = self.window_by_id(ipc_window.id) {
                        construct_window_object(&ipc_window, &window);

                        self.update_active_windows();

                        self.obj()
                            .emit_by_name::<()>("windows-changed", &[&*self.windows.borrow()]);
                    } else {
                        let new_window = super::Window::new();
                        construct_window_object(&ipc_window, &new_window);

                        if ipc_window.is_focused {
                            if let Some(window) = self.focused_window.borrow().as_ref() {
                                window.imp().is_focused.set(false);
                                window.notify_is_focused();
                                window.emit_by_name::<()>("changed", &[]);
                            }

                            self.focused_window.replace(Some(new_window.clone()));
                            self.obj().notify_focused_window();
                        }

                        self.windows.borrow_mut().append(&new_window);

                        self.update_active_windows();

                        self.obj()
                            .emit_by_name::<()>("windows-changed", &[&*self.windows.borrow()]);
                    }
                }
                niri_ipc::Event::WindowClosed { id } => {
                    if let Some(window) = self.window_by_id(id) {
                        self.windows
                            .borrow()
                            .remove(self.windows.borrow().find(&window).unwrap());

                        self.obj()
                            .emit_by_name::<()>("windows-changed", &[&*self.windows.borrow()]);
                    }
                }
                niri_ipc::Event::WorkspacesChanged { workspaces } => {
                    self.workspaces.borrow_mut().remove_all();

                    for window in self.active_windows.borrow().iter::<Window>() {
                        let _ = window.map(|window| {
                            window.imp().is_active.set(false);
                            window.notify_is_active();
                            window.emit_by_name::<()>("changed", &[]);
                        });
                    }
                    self.active_windows.borrow_mut().remove_all();

                    for workspace in workspaces {
                        let new_workspace = Workspace::new();

                        new_workspace.imp().id.set(workspace.id);
                        new_workspace.notify_id();
                        new_workspace.imp().idx.set(workspace.idx);
                        new_workspace.notify_idx();
                        new_workspace.imp().name.replace(workspace.name);
                        new_workspace.notify_name();
                        new_workspace.imp().output.replace(workspace.output);
                        new_workspace.notify_output();
                        new_workspace.imp().is_active.set(workspace.is_active);
                        new_workspace.notify_is_active();
                        new_workspace.imp().is_focused.set(workspace.is_focused);
                        new_workspace.notify_is_focused();
                        new_workspace
                            .imp()
                            .active_window_id
                            .set(workspace.active_window_id.into());
                        new_workspace.notify_active_window_id();

                        new_workspace.emit_by_name::<()>("changed", &[]);

                        self.workspaces.borrow_mut().append(&new_workspace);
                    }

                    self.update_active_windows();

                    self.obj()
                        .emit_by_name::<()>("workspaces-changed", &[&*self.workspaces.borrow()]);
                }
                niri_ipc::Event::WorkspaceActivated { id, focused } => {
                    for workspace in self.workspaces.borrow().iter::<Workspace>() {
                        let _ = workspace.map(|workspace| {
                            if workspace.id() == id {
                                let changed =
                                    !workspace.is_active() || workspace.is_focused() != focused;
                                if changed {
                                    workspace.imp().is_active.set(true);
                                    workspace.notify_is_active();
                                    workspace.imp().is_focused.set(focused);
                                    workspace.notify_is_focused();
                                    workspace.emit_by_name::<()>("changed", &[]);
                                }
                            } else {
                                let changed = workspace.is_active() || workspace.is_focused();
                                if changed {
                                    workspace.imp().is_active.set(false);
                                    workspace.imp().is_focused.set(false);
                                    workspace.notify_is_active();
                                    workspace.emit_by_name::<()>("changed", &[]);
                                }
                            }
                        });
                    }
                }
                _ => {}
            }
        }
    }

    impl Default for NiriService {
        fn default() -> Self {
            Self {
                // outputs: RefCell::new(ListStore::with_type(Output::static_type())),
                windows: RefCell::new(ListStore::with_type(super::Window::static_type())),
                workspaces: RefCell::new(ListStore::with_type(super::Workspace::static_type())),
                focused_window: Default::default(),
                active_windows: RefCell::new(ListStore::with_type(super::Window::static_type())),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for NiriService {
        const NAME: &'static str = "BalladServicesNiriService";
        type Type = super::NiriService;
    }

    #[glib::derived_properties]
    impl ObjectImpl for NiriService {
        fn constructed(&self) {
            self.parent_constructed();

            let Ok(niri_socket_path) = std::env::var("NIRI_SOCKET") else {
                println!("NIRI_SOCKET not set. Disabling Niri service.");
                return;
            };

            glib::spawn_future_local(clone!(
                #[weak(rename_to = this)]
                self,
                async move {
                    let mut stream = UnixStream::connect(niri_socket_path)
                        .await
                        .expect("Failed to connect to Niri socket.");
                    stream
                        .write_all(&serde_json::to_vec(&niri_ipc::Request::EventStream).unwrap())
                        .await
                        .expect("Failed to request an event stream from the Niri socket.");
                    stream
                        .shutdown(Shutdown::Write)
                        .expect("Failed to shutdown the write end of the Niri socket.");
                    let mut reader = BufReader::new(stream);
                    let mut line = String::new();
                    reader
                        .read_line(&mut line)
                        .await
                        .expect("Failed to get an event stream from the Niri socket.");
                    let Ok(niri_ipc::Response::Handled) =
                        serde_json::from_str::<niri_ipc::Reply>(&line)
                            .expect("Failed to parse Niri event stream response.")
                    else {
                        println!("Failed to get an event stream from the Niri socket.");
                        return;
                    };
                    let mut event_stream = reader.lines().map(|line| {
                        serde_json::from_str(&line.unwrap()).expect("Failed to parse Niri event.")
                    });
                    while let Some(event) = event_stream.next().await {
                        this.handle_niri_event(event);
                    }
                }
            ));
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    Signal::builder("windows-changed")
                        .param_types([ListStore::static_type()])
                        .build(),
                    Signal::builder("workspaces-changed")
                        .param_types([ListStore::static_type()])
                        .build(),
                    Signal::builder("focused-window-changed")
                        .param_types([super::Window::static_type()])
                        .build(),
                ]
            })
        }
    }
}

glib::wrapper! {
    pub struct NiriService(ObjectSubclass<imp::NiriService>);
}
impl NiriService {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
impl Default for NiriService {
    fn default() -> Self {
        Self::new()
    }
}

glib::wrapper! {
    pub struct Window(ObjectSubclass<window_imp::Window>);
}
impl Window {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
impl Default for Window {
    fn default() -> Self {
        Self::new()
    }
}

glib::wrapper! {
    pub struct Workspace(ObjectSubclass<workspace_imp::Workspace>);
}
impl Workspace {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
impl Default for Workspace {
    fn default() -> Self {
        Self::new()
    }
}

thread_local! {
    pub static NIRI_SERVICE: LazyCell<NiriService> = LazyCell::new(NiriService::new);
}
