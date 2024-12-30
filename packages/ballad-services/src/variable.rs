use std::borrow::Borrow;

use gtk::{
    glib::{self, Variant, closure_local},
    prelude::{FromVariant, ObjectExt},
};

#[derive(Debug, Default, Clone, PartialEq, Eq, glib::Boxed, glib::Variant)]
#[boxed_type(name = "BalladServicesVariableInner")]
pub struct VariableInner {
    pub value: Option<Variant>,
}

impl From<Variant> for VariableInner {
    fn from(value: Variant) -> Self {
        Self { value: Some(value) }
    }
}
impl From<Option<Variant>> for VariableInner {
    fn from(value: Option<Variant>) -> Self {
        Self { value }
    }
}
impl Borrow<Option<Variant>> for VariableInner {
    fn borrow(&self) -> &Option<Variant> {
        &self.value
    }
}

mod imp {
    use std::{cell::RefCell, sync::OnceLock};

    use gtk::{
        glib::{self, Properties, Variant, subclass::Signal},
        prelude::*,
        subclass::prelude::*,
    };

    use super::VariableInner;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::Variable)]
    pub struct Variable {
        #[property(get, set = Variable::set_inner)]
        value: RefCell<VariableInner>,
    }
    impl Variable {
        fn set_inner(&self, value: VariableInner) {
            let last = self.value.replace(value.clone());
            if last != value {
                self.obj().notify_value();
                if let Some(value) = value.value {
                    self.obj().emit_by_name::<()>("value-changed", &[&value]);
                }
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Variable {
        const NAME: &'static str = "BalladServicesVariable";
        type Type = super::Variable;
    }

    #[glib::derived_properties]
    impl ObjectImpl for Variable {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    Signal::builder("value-changed")
                        .param_types([Variant::static_type()])
                        .build(),
                ]
            })
        }
    }
}

glib::wrapper! {
    pub struct Variable(ObjectSubclass<imp::Variable>);
}

impl Variable {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn with_value(value: Variant) -> Self {
        let this: Self = glib::Object::builder().build();
        this.set_value(VariableInner::from(value));
        this
    }

    pub fn set_value_typed<T: Into<Variant>>(&self, value: T) {
        self.set_value(VariableInner::from(value.into()));
    }
    pub fn value_typed<T: FromVariant>(&self) -> Option<T> {
        self.value().value.map(|v| v.get().unwrap())
    }

    pub fn connect_value_changed_typed<T: FromVariant, F: Fn(Variable, T) + 'static>(
        &self,
        after: bool,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_closure(
            "value-changed",
            after,
            closure_local!(move |variable: Variable, variant: Variant| {
                let value: T = variant.get().unwrap();
                f(variable, value);
            }),
        )
    }
}

impl Default for Variable {
    fn default() -> Self {
        Self::new()
    }
}
