use std::cell::LazyCell;

use gtk::glib;

mod imp {
    use std::cell::RefCell;
    use std::sync::OnceLock;

    use ballad_config::{ShellConfig, ThemeConfig};
    use gtk::gio::Cancellable;
    use gtk::glib::clone;
    use gtk::glib::subclass::Signal;
    use gtk::{gio, glib};
    use gtk::{glib::Properties, prelude::*, subclass::prelude::*};

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::ConfigService)]
    pub struct ConfigService {
        #[property(get)]
        shell_config_path: RefCell<String>,

        #[property(
            type = ShellConfig,
            name = "shell-config",
            get = |_| ballad_config::get_or_init_shell_config(),
            set = |_, val| ballad_config::set_shell_config(val)
        )]
        _shell_config: (),

        last_config: RefCell<ShellConfig>,

        shell_config_watcher: RefCell<Option<gio::FileMonitor>>,
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

            let shell_config_path: String =
                ballad_config::shell_config_path().to_string_lossy().into();
            self.shell_config_path.replace(shell_config_path.clone());

            let shell_config_file = gio::File::for_path(&shell_config_path);
            let shell_config_watcher = shell_config_file
                .monitor(gio::FileMonitorFlags::NONE, Cancellable::NONE)
                .unwrap();

            self.last_config.replace(self.obj().shell_config());

            shell_config_watcher.connect_changed(clone!(
                #[weak(rename_to = this)]
                self,
                move |_, _, _, event| {
                    if event == gio::FileMonitorEvent::ChangesDoneHint {
                        let config = this.obj().shell_config();
                        this.obj()
                            .emit_by_name::<()>("shell-config-changed", &[&config]);

                        if config.theme != this.last_config.borrow().theme {
                            this.obj()
                                .emit_by_name::<()>("shell-theme-config-changed", &[&config.theme]);
                        }

                        this.obj().notify_shell_config();

                        this.last_config.replace(config);
                    }
                }
            ));

            self.shell_config_watcher
                .replace(Some(shell_config_watcher));
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    Signal::builder("shell-config-changed")
                        .param_types([ShellConfig::static_type()])
                        .build(),
                    Signal::builder("shell-theme-config-changed")
                        .param_types([ThemeConfig::static_type()])
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
