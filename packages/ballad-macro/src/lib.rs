#![feature(proc_macro_diagnostic)]

mod parse;

use parse::{Get, GetOrSetValue, Set};
use proc_macro::{Diagnostic, TokenStream};
use quote::{format_ident, quote};
use syn::{parse_macro_input, spanned::Spanned};

#[proc_macro_derive(Reactive, attributes(wrapper_type, property))]
pub fn derive_reactive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let syn::Data::Struct(ref data) = input.data else {
        return syn::Error::new_spanned(input, "Expected struct")
            .into_compile_error()
            .into();
    };

    let Some(wrapper_type) = input.attrs.iter().find_map(|attr| {
        attr.path()
            .is_ident("wrapper_type")
            .then(|| attr.parse_args::<syn::Path>().unwrap())
    }) else {
        return syn::Error::new_spanned(input, "Expected #[wrapper_type(<type>)]")
            .into_compile_error()
            .into();
    };
    let input_ident = input.ident;

    let prop_impls = data.fields.iter().filter_map(|field| {
        let field_ident = field.ident.as_ref()?;

        let property_attr = field.attrs.iter().find(|attr| attr.path().is_ident("property"))?;
        let property_attr_data = property_attr.parse_args::<parse::Property>().unwrap();

        if property_attr_data.set.is_none() && property_attr_data.get.is_none() {
            Diagnostic::spanned(property_attr.span().unwrap(), proc_macro::Level::Warning, "Property attr has no getter or setter. This will not generate code!").emit();
            return None;
        }

        let ty = &field.ty;

        let blocking_ident = format_ident!("{field_ident}_blocking");

        let get = match property_attr_data.get {
            Some(Get {
                value: Some(GetOrSetValue { value, .. }),
                ..
            }) => {
                let getter = value;
                quote! {
                    pub fn #field_ident(&self) -> #ty {
                        let getter = #getter;
                        getter(self)
                    }
                    pub fn #blocking_ident(&self) -> #ty {
                        smol::block_on(self.#field_ident())
                    }
                }
            },
            Some(Get { .. }) => {
                quote! {
                    pub async fn #field_ident(&self) -> #ty {
                        self.inner.get().await.#field_ident
                    }
                    pub fn #blocking_ident(&self) -> #ty {
                        self.inner.get_blocking().#field_ident
                    }
                }
            },
            None => quote! {}
        };

        let ident = format_ident!("set_{field_ident}");
        let blocking_ident = format_ident!("set_{field_ident}_blocking");

        
        let set = match property_attr_data.set {
            Some(Set {
                value: Some(GetOrSetValue { value, .. }),
                ..
            }) => {
                let setter = value;
                
                quote! {
                    pub async fn #ident(&self, #field_ident: #ty) {
                        let setter = #setter;
                        setter(self, #field_ident)
                    }
                    pub fn #blocking_ident(&self, value: #ty) {
                        smol::block_on(self.#ident(value))
                    }
                }
            },
            Some(Set { .. }) => {
                quote! {
                    pub async fn #ident(&self, #field_ident: #ty) {
                        let this = self.inner.get().await;
                        self.inner.set(#input_ident {
                            #field_ident,
                            ..this
                        }).await
                    }
                    pub fn #blocking_ident(&self, value: #ty) {
                        let this = self.inner.get_blocking();
                        self.inner.set_blocking(#input_ident {
                            #field_ident,
                            ..this
                        })
                    }
                }
            },
            None => quote! {}
        };

        let connect_ident = format_ident!("connect_{field_ident}");
        Some(quote! {
            impl #wrapper_type {
                #get
                #set

                pub fn #connect_ident(&self, connection: impl Fn(Self, #ty) + 'static) {
                    let this = self.clone();
                    let cached = std::cell::RefCell::new(self.inner.get_blocking().#field_ident);
                    self.inner.connect(move |_, value| {
                        if value.#field_ident != *cached.borrow() {
                            *cached.borrow_mut() = value.#field_ident.clone();
                            connection(this.clone(), value.#field_ident)
                        }
                    })
                }
            }
        })
    });

    quote! {
        #(#prop_impls)*

        impl #wrapper_type {
            pub fn connect(&self, connection: impl Fn(Self) + 'static) {
                let this = self.clone();
                self.inner.connect(move |_, value| {
                    connection(this.clone())
                })
            }
        }
    }
    .into()
}
