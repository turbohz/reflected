use std::str::FromStr;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::Parser,
    parse_macro_input, Data, DeriveInput, Fields, FieldsNamed, Ident, Type,
    __private::{Span, TokenStream2},
};

use crate::field::Field;

mod field;

#[proc_macro_attribute]
pub fn reflected(_args: TokenStream, stream: TokenStream) -> TokenStream {
    let mut stream = parse_macro_input!(stream as DeriveInput);

    let data = match &mut stream.data {
        Data::Struct(data) => data,
        _ => panic!("`db_entity` macro has to be used with structs"),
    };

    let struct_fields = match &mut data.fields {
        Fields::Named(fields) => fields,
        _ => panic!(),
    };

    let fields = parse_fields(struct_fields);

    let name = &stream.ident;
    let name_string = TokenStream2::from_str(&format!("\"{}\"", name)).unwrap();

    let fields_struct_name = Ident::new(&format!("{}Fields", name), Span::call_site());

    let fields_struct = fields_struct(&fields);
    let fields_const_var = fields_const_var(&fields);
    let fields_reflect = fields_reflect(name, &fields);
    let fields_get_value = fields_get_value(&fields);
    let fields_set_value = fields_set_value(&fields);

    struct_fields.named.push(
        syn::Field::parse_named
            .parse2(quote! { pub id: Option<u64> })
            .unwrap(),
    );

    quote! {
        #stream

        use std::borrow::Borrow;
        use database::{to_database_string, TryIntoVal};

        pub struct #fields_struct_name {
            #fields_struct
        }

        impl #name {
            pub const FIELDS: #fields_struct_name = #fields_struct_name {
                id: database::Field {
                    name: "rowid",
                    tp: reflected::Type::Integer,
                    unique: false,
                },
                #fields_const_var
            };
        }

        impl database::Reflected for #name {
            fn type_name() -> &'static str {
                #name_string
            }

            fn fields() -> &'static [database::Field] {
                &[
                    database::Field {
                        name: "rowid",
                        tp: database::Type::Integer,
                        unique: false,
                    },
                    #fields_reflect
                ]
            }

            fn get_value(&self, field: impl Borrow<database::Field>) -> String {
                let field = field.borrow();
                match field.name {
                    #fields_get_value
                    "rowid" => to_database_string(&self.id.unwrap_or_default(), false),
                    _ => unreachable!("Invalid field value in get_value: {}", field.name),
                }
            }

            fn set_value(&mut self, value: &str, field: &'static database::Field) {
                match field.name {
                    #fields_set_value
                    "rowid" => self.id = value.try_into_val(),
                    _ => unreachable!("Invalid field value in set_value"),
                }
            }
        }
    }
    .into()
}

fn fields_const_var(fields: &Vec<Field>) -> TokenStream2 {
    let mut res = quote!();

    for field in fields {
        let name = &field.name;

        let Some(field_type) = field.field_type() else {
            continue
        };

        let name_string = field.name_as_string();

        res = quote! {
            #res
            #name: database::Field {
                name: #name_string,
                tp: database::Type::#field_type,
                unique: false,
            },
        }
    }

    res
}

fn fields_struct(fields: &Vec<Field>) -> TokenStream2 {
    let mut res = quote!();

    for field in fields {
        if !field.db_serializable() {
            continue;
        }
        let name = &field.name;
        res = quote! {
            #res
            pub #name: database::Field,
        }
    }

    quote! {
        pub id: database::Field,
        #res
    }
}

fn fields_reflect(name: &Ident, fields: &Vec<Field>) -> TokenStream2 {
    let mut res = quote!();

    for field in fields {
        if !field.db_serializable() {
            continue;
        }
        let field_name = &field.name;
        res = quote! {
            #res
            #name::FIELDS.#field_name,
        }
    }

    res
}

fn fields_get_value(fields: &Vec<Field>) -> TokenStream2 {
    let mut res = quote!();

    for field in fields {
        if !field.db_serializable() {
            continue;
        }

        let field_name = &field.name;
        let name_string = field.name_as_string();

        res = quote! {
            #res
            #name_string => to_database_string(&self.#field_name, field.is_text()),
        }
    }

    res
}

fn fields_set_value(fields: &Vec<Field>) -> TokenStream2 {
    let mut res = quote!();

    for field in fields {
        if !field.db_serializable() {
            continue;
        }

        let field_name = &field.name;
        let name_string = field.name_as_string();

        res = quote! {
            #res
            #name_string => self.#field_name = value.try_into_val(),
        }
    }

    res
}

fn parse_fields(fields: &FieldsNamed) -> Vec<Field> {
    fields
        .named
        .iter()
        .map(|field| {
            let name = field.ident.as_ref().unwrap();

            let path = match &field.ty {
                Type::Path(path) => path,
                _ => unreachable!("invalid parse_fields"),
            };

            let tp = &path.path.segments.first().unwrap().ident;

            Field {
                name: name.clone(),
                tp:   tp.clone(),
            }
        })
        .collect()
}
