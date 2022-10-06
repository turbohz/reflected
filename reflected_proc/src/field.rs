use std::str::FromStr;

use syn::{Ident, __private::Span};

use crate::TokenStream2;

#[derive(Debug)]
pub(crate) struct Field {
    pub(crate) name: Ident,
    pub(crate) tp: Ident,
}

impl Field {
    pub(crate) fn name_as_string(&self) -> TokenStream2 {
        TokenStream2::from_str(&format!("\"{}\"", self.name)).unwrap()
    }

    pub(crate) fn db_serializable(&self) -> bool {
        self.field_type().is_some()
    }

    pub(crate) fn field_type(&self) -> Option<Ident> {
        let integer = Ident::new("Integer", Span::call_site());
        let text = Ident::new("Text", Span::call_site());

        match self.tp.to_string().as_str() {
            "i32" => integer.into(),
            "u64" => integer.into(),
            "usize" => integer.into(),
            "String" => text.into(),
            _ => None,
        }
    }
}
