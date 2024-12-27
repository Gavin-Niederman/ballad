use std::cell::LazyCell;

use gtk::glib;

mod imp {
    use std::cell::RefCell;
    use std::sync::OnceLock;

    use ballad_config::ShellConfig;
    use gtk::gio::Cancellable;
    use gtk::glib::clone;
    use gtk::glib::subclass::Signal;
    use gtk::{gio, glib};
    use gtk::{glib::Properties, prelude::*, subclass::prelude::*};

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::ConfigService)]
    pub struct ConfigService {
        #[property(get)]
        config_path: RefCell<String>,

        #[property(
            type = ShellConfig,
            name = "config",
            get = |_| ballad_config::get_or_init_shell_config(),
            set = |_, val| ballad_config::set_shell_config(val)
        )]
        _config: (),

        watcher: RefCell<Option<gio::FileMonitor>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ConfigService {
        const NAME: &'static str = "BalladServicesConfigService";
        type Type = super::ConfigService;
    }

    #[glib::derived_properties]
    impl ObjectImpl for ConfigService {
        fn constructed(&self) {
            self.parent_constructed();

            let config_path: String = ballad_config::shell_config_path().to_string_lossy().into();
            self.config_path.replace(config_path.clone());

            let file = gio::File::for_path(&config_path);
            let watcher = file
                .monitor(gio::FileMonitorFlags::NONE, Cancellable::NONE)
                .unwrap();

            watcher.connect_changed(clone!(
                #[weak(rename_to = this)]
                self,
                move |_, _, _, event| {
                    if event == gio::FileMonitorEvent::ChangesDoneHint {
                        let config = this.obj().config();
                        this.obj().emit_by_name::<()>("config-changed", &[&config]);
                        this.obj().notify_config();
                    }
                }
            ));

            self.watcher.replace(Some(watcher));
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    Signal::builder("config-changed")
                        .param_types([ShellConfig::static_type()])
                        .build(),
                ]
            })
        }
    }
}

glib::wrapper! {
    pub struct ConfigService(ObjectSubclass<imp::ConfigService>);
}

impl ConfigService {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
impl Default for ConfigService {
    fn default() -> Self {
        Self::new()
    }
}

thread_local! {
    pub static CONFIG_SERVICE: LazyCell<ConfigService> = LazyCell::new(ConfigService::new);
}
