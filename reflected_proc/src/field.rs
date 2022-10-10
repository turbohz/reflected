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

    pub(crate) fn supported(&self) -> bool {
        self.field_type() != Ident::new("Unsupported", Span::call_site())
    }

    pub(crate) fn field_type(&self) -> Ident {
        let integer = Ident::new("Integer", Span::call_site());
        let text = Ident::new("Text", Span::call_site());
        let unsupported = Ident::new("Unsupported", Span::call_site());

        match self.tp.to_string().as_str() {
            "i32" => integer,
            "u64" => integer,
            "usize" => integer,
            "String" => text,
            _ => unsupported,
        }
    }
}
