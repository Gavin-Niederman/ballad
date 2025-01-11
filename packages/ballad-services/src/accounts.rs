use std::cell::LazyCell;

use bus::UserProxy;
use gtk::{glib, subclass::prelude::ObjectSubclassIsExt};
use zbus::zvariant::OwnedObjectPath;

use crate::DBUS_SYSTEM_CONNECTION;

mod bus {
    //! # D-Bus interface proxies for: `org.freedesktop.Accounts` and `org.freedesktop.Accounts.User`
    //!
    //! This code was generated by `zbus-xmlgen` `4.1.0` from D-Bus introspection data.
    #![allow(clippy::type_complexity)]

    use zbus::proxy;
    #[proxy(interface = "org.freedesktop.Accounts", assume_defaults = true)]
    pub trait Accounts {
        /// CacheUser method
        fn cache_user(&self, name: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

        /// CreateUser method
        fn create_user(
            &self,
            name: &str,
            fullname: &str,
            account_type: i32,
        ) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

        /// DeleteUser method
        fn delete_user(&self, id: i64, remove_files: bool) -> zbus::Result<()>;

        /// FindUserById method
        fn find_user_by_id(&self, id: i64) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

        /// FindUserByName method
        fn find_user_by_name(&self, name: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

        /// GetUsersLanguages method
        fn get_users_languages(&self) -> zbus::Result<Vec<String>>;

        /// ListCachedUsers method
        fn list_cached_users(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

        /// UncacheUser method
        fn uncache_user(&self, name: &str) -> zbus::Result<()>;

        /// UserAdded signal
        #[zbus(signal)]
        fn user_added(&self, user: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

        /// UserDeleted signal
        #[zbus(signal)]
        fn user_deleted(&self, user: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

        /// AutomaticLoginUsers property
        #[zbus(property)]
        fn automatic_login_users(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

        /// DaemonVersion property
        #[zbus(property)]
        fn daemon_version(&self) -> zbus::Result<String>;

        /// HasMultipleUsers property
        #[zbus(property)]
        fn has_multiple_users(&self) -> zbus::Result<bool>;

        /// HasNoUsers property
        #[zbus(property)]
        fn has_no_users(&self) -> zbus::Result<bool>;
    }

    #[proxy(
        interface = "org.freedesktop.Accounts.User",
        default_service = "org.freedesktop.Accounts"
    )]
    pub trait User {
        /// GetPasswordExpirationPolicy method
        fn get_password_expiration_policy(&self) -> zbus::Result<(i64, i64, i64, i64, i64, i64)>;

        /// SetAccountType method
        fn set_account_type(&self, account_type: i32) -> zbus::Result<()>;

        /// SetAutomaticLogin method
        fn set_automatic_login(&self, enabled: bool) -> zbus::Result<()>;

        /// SetEmail method
        fn set_email(&self, email: &str) -> zbus::Result<()>;

        /// SetHomeDirectory method
        fn set_home_directory(&self, homedir: &str) -> zbus::Result<()>;

        /// SetIconFile method
        fn set_icon_file(&self, filename: &str) -> zbus::Result<()>;

        /// SetLanguage method
        fn set_language(&self, language: &str) -> zbus::Result<()>;

        /// SetLanguages method
        fn set_languages(&self, languages: &[&str]) -> zbus::Result<()>;

        /// SetLocation method
        fn set_location(&self, location: &str) -> zbus::Result<()>;

        /// SetLocked method
        fn set_locked(&self, locked: bool) -> zbus::Result<()>;

        /// SetPassword method
        fn set_password(&self, password: &str, hint: &str) -> zbus::Result<()>;

        /// SetPasswordExpirationPolicy method
        fn set_password_expiration_policy(
            &self,
            min_days_between_changes: i64,
            max_days_between_changes: i64,
            days_to_warn: i64,
            days_after_expiration_until_lock: i64,
        ) -> zbus::Result<()>;

        /// SetPasswordHint method
        fn set_password_hint(&self, hint: &str) -> zbus::Result<()>;

        /// SetPasswordMode method
        fn set_password_mode(&self, mode: i32) -> zbus::Result<()>;

        /// SetRealName method
        fn set_real_name(&self, name: &str) -> zbus::Result<()>;

        /// SetSession method
        fn set_session(&self, session: &str) -> zbus::Result<()>;

        /// SetSessionType method
        fn set_session_type(&self, session_type: &str) -> zbus::Result<()>;

        /// SetShell method
        fn set_shell(&self, shell: &str) -> zbus::Result<()>;

        /// SetUserExpirationPolicy method
        fn set_user_expiration_policy(&self, expiration_time: i64) -> zbus::Result<()>;

        /// SetUserName method
        fn set_user_name(&self, name: &str) -> zbus::Result<()>;

        /// SetXSession method
        #[zbus(name = "SetXSession")]
        fn set_xsession(&self, x_session: &str) -> zbus::Result<()>;

        /// Changed signal
        #[zbus(signal)]
        fn changed(&self) -> zbus::Result<()>;

        /// AccountType property
        #[zbus(property)]
        fn account_type(&self) -> zbus::Result<i32>;

        /// AutomaticLogin property
        #[zbus(property)]
        fn automatic_login(&self) -> zbus::Result<bool>;

        /// Email property
        #[zbus(property)]
        fn email(&self) -> zbus::Result<String>;

        /// HomeDirectory property
        #[zbus(property)]
        fn home_directory(&self) -> zbus::Result<String>;

        /// IconFile property
        #[zbus(property)]
        fn icon_file(&self) -> zbus::Result<String>;

        /// Language property
        #[zbus(property)]
        fn language(&self) -> zbus::Result<String>;

        /// Languages property
        #[zbus(property)]
        fn languages(&self) -> zbus::Result<Vec<String>>;

        /// LocalAccount property
        #[zbus(property)]
        fn local_account(&self) -> zbus::Result<bool>;

        /// Location property
        #[zbus(property)]
        fn location(&self) -> zbus::Result<String>;

        /// Locked property
        #[zbus(property)]
        fn locked(&self) -> zbus::Result<bool>;

        /// LoginFrequency property
        #[zbus(property)]
        fn login_frequency(&self) -> zbus::Result<u64>;

        /// LoginHistory property
        #[zbus(property)]
        fn login_history(
            &self,
        ) -> zbus::Result<
            Vec<(
                i64,
                i64,
                std::collections::HashMap<String, zbus::zvariant::OwnedValue>,
            )>,
        >;

        /// LoginTime property
        #[zbus(property)]
        fn login_time(&self) -> zbus::Result<i64>;

        /// PasswordHint property
        #[zbus(property)]
        fn password_hint(&self) -> zbus::Result<String>;

        /// PasswordMode property
        #[zbus(property)]
        fn password_mode(&self) -> zbus::Result<i32>;

        /// RealName property
        #[zbus(property)]
        fn real_name(&self) -> zbus::Result<String>;

        /// Saved property
        #[zbus(property)]
        fn saved(&self) -> zbus::Result<bool>;

        /// Session property
        #[zbus(property)]
        fn session(&self) -> zbus::Result<String>;

        /// SessionType property
        #[zbus(property)]
        fn session_type(&self) -> zbus::Result<String>;

        /// Shell property
        #[zbus(property)]
        fn shell(&self) -> zbus::Result<String>;

        /// SystemAccount property
        #[zbus(property)]
        fn system_account(&self) -> zbus::Result<bool>;

        /// Uid property
        #[zbus(property)]
        fn uid(&self) -> zbus::Result<u64>;

        /// UserName property
        #[zbus(property)]
        fn user_name(&self) -> zbus::Result<String>;

        /// UsesHomed property
        #[zbus(property)]
        fn uses_homed(&self) -> zbus::Result<bool>;

        /// XSession property
        #[zbus(property, name = "XSession")]
        fn xsession(&self) -> zbus::Result<String>;
    }
}

mod user_imp {
    use std::cell::{Cell, RefCell};
    use std::path::PathBuf;
    use std::sync::OnceLock;

    use futures::join;
    use gtk::glib;
    use gtk::glib::Properties;
    use gtk::glib::subclass::Signal;
    use gtk::{prelude::*, subclass::prelude::*};
    use smol::lock::RwLock;

    use super::bus::UserProxy;

    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, glib::Enum)]
    #[repr(i32)]
    #[enum_type(name = "BalladServicesAccountType")]
    pub enum AccountType {
        #[default]
        Standard = 0,
        Administrator = 1,
    }

    #[derive(Default, Properties)]
    #[properties(wrapper_type = super::User)]
    pub struct User {
        #[property(get, builder(AccountType::Standard))]
        account_type: Cell<AccountType>,

        #[property(get)]
        icon_file: RefCell<Option<String>>,
        #[property(get)]
        email: RefCell<Option<String>>,
        #[property(get)]
        real_name: RefCell<Option<String>>,

        #[property(get)]
        login_frequency: Cell<u64>,

        #[property(get)]
        user_name: RefCell<String>,
        #[property(get)]
        uid: Cell<u64>,

        pub(super) proxy: RwLock<Option<UserProxy<'static>>>,
    }
    impl User {
        pub async fn update(&self) {
            let proxy = self.proxy.read().await;
            let proxy = proxy.as_ref().unwrap();

            let (account_type, icon_file, email, real_name, login_frequency, user_name, uid) = join!(
                proxy.account_type(),
                proxy.icon_file(),
                proxy.email(),
                proxy.real_name(),
                proxy.login_frequency(),
                proxy.user_name(),
                proxy.uid(),
            );

            self.account_type
                .set(match account_type.unwrap_or_default() {
                    0 => AccountType::Standard,
                    1 => AccountType::Administrator,
                    _ => AccountType::Standard,
                });

            fn none_if_empty(s: String) -> Option<String> {
                if s.is_empty() { None } else { Some(s) }
            }

            self.obj().notify_account_type();
            self.icon_file
                .replace(none_if_empty(icon_file.unwrap_or_default()));
            self.obj().notify_icon_file();
            self.email.replace(none_if_empty(email.unwrap_or_default()));
            self.obj().notify_email();
            self.real_name
                .replace(none_if_empty(real_name.unwrap_or_default()));
            self.obj().notify_real_name();

            self.login_frequency
                .set(login_frequency.unwrap_or_default());
            self.obj().notify_login_frequency();
            self.user_name.replace(user_name.unwrap_or_default());
            self.obj().notify_user_name();
            self.uid.set(uid.unwrap_or_default());
            self.obj().notify_uid();

            self.obj().emit_by_name::<()>("changed", &[]);
        }

        pub async fn set_icon(&self, path: PathBuf) -> zbus::Result<()> {
            let proxy = self.proxy.read().await;
            let proxy = proxy.as_ref().unwrap();
            proxy.set_icon_file(path.to_string_lossy().as_ref()).await?;
            self.icon_file
                .replace(Some(path.to_string_lossy().to_string()));
            self.obj().notify_icon_file();

            Ok(())
        }
        pub async fn set_email(&self, email: &str) -> zbus::Result<()> {
            let proxy = self.proxy.read().await;
            let proxy = proxy.as_ref().unwrap();
            proxy.set_email(email).await?;
            self.email.replace(Some(email.to_string()));
            self.obj().notify_email();

            Ok(())
        }
        pub async fn set_real_name(&self, name: &str) -> zbus::Result<()> {
            let proxy = self.proxy.read().await;
            let proxy = proxy.as_ref().unwrap();
            proxy.set_real_name(name).await?;
            self.real_name.replace(Some(name.to_string()));
            self.obj().notify_real_name();

            Ok(())
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for User {
        const NAME: &'static str = "BalladServicesUser";
        type Type = super::User;
    }

    #[glib::derived_properties]
    impl ObjectImpl for User {
        fn constructed(&self) {
            self.parent_constructed();
        }
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| vec![Signal::builder("changed").build()])
        }
    }
}

mod imp {
    use std::sync::OnceLock;

    use gtk::gio::ListStore;
    use gtk::glib;
    use gtk::glib::Properties;
    use gtk::glib::subclass::Signal;
    use gtk::{prelude::*, subclass::prelude::*};
    use smol::lock::RwLock;

    use crate::DBUS_SYSTEM_CONNECTION;

    use super::User;
    use super::bus::AccountsProxy;

    #[derive(Properties)]
    #[properties(wrapper_type = super::AccountsService)]
    pub struct AccountsService {
        #[property(get)]
        cached_users: ListStore,

        proxy: RwLock<Option<AccountsProxy<'static>>>,
    }
    impl Default for AccountsService {
        fn default() -> Self {
            Self {
                cached_users: ListStore::with_type(User::static_type()),

                proxy: Default::default(),
            }
        }
    }

    impl AccountsService {
        pub async fn update(&self) {
            let proxy = self.proxy.read().await;
            let proxy = proxy.as_ref().unwrap();

            self.cached_users.remove_all();
            let users = proxy.list_cached_users().await.unwrap();
            for user in users {
                let user = User::with_path(user).await;
                self.cached_users.append(&user);
            }
            self.obj().notify_cached_users();
            self.obj().emit_by_name::<()>("cached-users-changed", &[]);
        }

        pub async fn cache_user(&self, name: &str) -> zbus::Result<()> {
            let proxy = self.proxy.read().await;
            let proxy = proxy.as_ref().unwrap();
            proxy.cache_user(name).await?;

            self.update().await;

            Ok(())
        }
        pub async fn uncache_user(&self, name: &str) -> zbus::Result<()> {
            let proxy = self.proxy.read().await;
            let proxy = proxy.as_ref().unwrap();
            proxy.uncache_user(name).await?;

            self.update().await;

            Ok(())
        }

        pub async fn find_user_by_name(&self, name: &str) -> Option<User> {
            let proxy = self.proxy.read().await;
            let proxy = proxy.as_ref()?;

            let user_path = proxy.find_user_by_name(name).await.ok()?;
            let user = User::with_path(user_path).await;

            Some(user)
        }
        pub async fn find_user_by_id(&self, id: u64) -> Option<User> {
            let proxy = self.proxy.read().await;
            let proxy = proxy.as_ref()?;

            let user_path = proxy.find_user_by_id(id as _).await.ok()?;
            let user = User::with_path(user_path).await;

            Some(user)
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AccountsService {
        const NAME: &'static str = "BalladServicesAccountsService";
        type Type = super::AccountsService;
    }

    #[glib::derived_properties]
    impl ObjectImpl for AccountsService {
        fn constructed(&self) {
            self.parent_constructed();
            smol::block_on(async {
                let Ok(proxy) = AccountsProxy::new(&DBUS_SYSTEM_CONNECTION).await else {
                    println!("Failed to create AccountsProxy. Accounts service will not function!");
                    return;
                };
                self.proxy.write().await.replace(proxy);

                self.update().await;
            })
        }
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| vec![Signal::builder("cached-users-changed").build()])
        }
    }
}

glib::wrapper! {
    pub struct AccountsService(ObjectSubclass<imp::AccountsService>);
}
impl AccountsService {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub async fn cache_user(&self, name: &str) -> zbus::Result<()> {
        self.imp().cache_user(name).await
    }
    pub async fn uncache_user(&self, name: &str) -> zbus::Result<()> {
        self.imp().uncache_user(name).await
    }

    pub async fn find_user_by_name(&self, name: &str) -> Option<User> {
        self.imp().find_user_by_name(name).await
    }
    pub async fn find_user_by_id(&self, id: u64) -> Option<User> {
        self.imp().find_user_by_id(id).await
    }
}
impl Default for AccountsService {
    fn default() -> Self {
        Self::new()
    }
}

glib::wrapper! {
    pub struct User(ObjectSubclass<user_imp::User>);
}
impl User {
    pub async fn with_path(path: OwnedObjectPath) -> Self {
        let proxy = UserProxy::new(&DBUS_SYSTEM_CONNECTION, path).await.unwrap();
        let user: Self = glib::Object::builder().build();
        user.imp().proxy.write().await.replace(proxy);

        user.imp().update().await;

        user
    }

    pub async fn set_icon(&self, path: std::path::PathBuf) -> zbus::Result<()> {
        self.imp().set_icon(path).await
    }
    pub async fn set_email(&self, email: &str) -> zbus::Result<()> {
        self.imp().set_email(email).await
    }
    pub async fn set_real_name(&self, name: &str) -> zbus::Result<()> {
        self.imp().set_real_name(name).await
    }
}

thread_local! {
    pub static ACCOUNTS_SERVICE: LazyCell<AccountsService> = LazyCell::new(AccountsService::new)
}
