use syn::{parse::{Parse, ParseStream}, Token};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(get);
    custom_keyword!(set);
}

pub struct GetOrSetValue {
    pub _eq: Token![=],
    pub value: syn::Expr,
}
impl Parse for GetOrSetValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(GetOrSetValue {
            _eq: input.parse()?,
            value: input.parse()?,
        })
    }
}

pub struct Get {
    pub _get: kw::get,
    pub value: Option<GetOrSetValue>,
}
impl Parse for Get {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let get = input.parse()?;
        let value = if input.peek(Token![=]) {
            Some(input.parse()?)
        } else {
            None
        };
        
        Ok(Get {
            _get: get,
            value,
        })
    }
}
pub struct Set {
    pub _set: kw::set,
    pub value: Option<GetOrSetValue>,
}
impl Parse for Set {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let set = input.parse()?;
        let value = if input.peek(Token![=]) {
            Some(input.parse()?)
        } else {
            None
        };
        
        Ok(Set {
            _set: set,
            value,
        })
    }
}

pub struct Property {
    pub get: Option<Get>,
    pub set: Option<Set>
}
impl Parse for Property {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let get = if input.peek(kw::get) {
            Some(input.parse()?)
        } else {
            None
        };
        _ = input.parse::<Token![,]>();
        let set = if input.peek(kw::set) {
            Some(input.parse()?)
        } else {
            None
        };
        Ok(Property { get, set })
    }
}