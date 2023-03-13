use std::str::FromStr;

use syn::{Ident, __private::Span};

use crate::TokenStream2;

#[derive(Debug)]
pub(crate) struct Field {
    pub(crate) name:   Ident,
    pub(crate) tp:     Ident,
    pub(crate) unique: bool,
    pub(crate) secure: bool,
}

impl Field {
    pub(crate) fn name_as_string(&self) -> TokenStream2 {
        TokenStream2::from_str(&format!("\"{}\"", self.name)).unwrap()
    }

    pub(crate) fn type_as_string(&self) -> TokenStream2 {
        TokenStream2::from_str(&format!("\"{}\"", self.tp)).unwrap()
    }

    pub(crate) fn id(&self) -> bool {
        self.name == Ident::new("id", Span::call_site())
    }

    pub(crate) fn custom(&self) -> bool {
        self.field_type() == Ident::new("Custom", Span::call_site())
    }

    pub(crate) fn is_simple(&self) -> bool {
        !self.id() && !self.custom()
    }

    pub(crate) fn field_type(&self) -> Ident {
        let float = Ident::new("Float", Span::call_site());
        let integer = Ident::new("Integer", Span::call_site());
        let text = Ident::new("Text", Span::call_site());
        let custom = Ident::new("Custom", Span::call_site());

        match self.tp.to_string().as_str() {
            "f32" | "f64" => float,
            "i32" | "u32" | "i64" | "u64" | "isize" | "usize" => integer,
            "String" => text,
            _ => custom,
        }
    }
}
