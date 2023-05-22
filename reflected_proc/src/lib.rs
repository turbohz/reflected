use std::str::FromStr;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Fields, FieldsNamed, Ident, Meta, NestedMeta, Type,
    __private::{Span, TokenStream2},
};

use crate::field::Field;

mod field;

/// Data must also derive `Default`
#[proc_macro_derive(Reflected, attributes(unique, secure, name))]
pub fn reflected(stream: TokenStream) -> TokenStream {
    let mut stream = parse_macro_input!(stream as DeriveInput);

    let data = match &mut stream.data {
        Data::Struct(data) => data,
        _ => panic!("`db_entity` macro has to be used with structs"),
    };

    let struct_fields = match &mut data.fields {
        Fields::Named(fields) => fields,
        _ => panic!(),
    };

    let (rename, fields) = parse_fields(struct_fields);

    let name = stream.ident.clone();

    let name_string = if let Some(rename) = rename {
        TokenStream2::from_str(&format!("\"{rename}\""))
    } else {
        TokenStream2::from_str(&format!("\"{name}\""))
    }
    .unwrap();

    let fields_struct_name = Ident::new(&format!("{name}Fields"), Span::call_site());

    let fields_struct = fields_struct(&fields);
    let fields_const_var = fields_const_var(&name, &fields);
    let fields_reflect = fields_reflect(&name, &fields);
    let simple_fields_reflect = simple_fields_reflect(&name, &fields);
    let fields_get_value = fields_get_value(&fields);
    let fields_set_value = fields_set_value(&fields);

    quote! {
        #[derive(Debug)]
        pub struct #fields_struct_name {
            #fields_struct
        }

        impl #name {
            pub const FIELDS: #fields_struct_name = #fields_struct_name {
                #fields_const_var
            };
        }

        impl reflected::Reflected for #name {
            fn type_name() -> &'static str {
                #name_string
            }

            fn fields() -> &'static [reflected::Field] {
                &[
                    #fields_reflect
                ]
            }

            fn simple_fields() -> &'static [reflected::Field] {
                &[
                    #simple_fields_reflect
                ]
            }

            fn get_value(&self, field: impl std::borrow::Borrow<reflected::Field>) -> String {
                use std::borrow::Borrow;
                let field = field.borrow();

                if field.is_custom() {
                    panic!("get_value method is not supported for custom types: {field:?}");
                }

                match field.name {
                    #fields_get_value
                    _ => unreachable!("Invalid field name in get_value: {}", field.name),
                }
            }

            fn set_value(&mut self, field: impl std::borrow::Borrow<reflected::Field>, value: &str) {
                use reflected::TryIntoVal;
                use std::borrow::Borrow;
                let field = field.borrow();
                match field.name {
                    #fields_set_value
                    _ => unreachable!("Invalid field name in set_value"),
                }
            }
        }
    }
    .into()
}

fn fields_const_var(type_name: &Ident, fields: &Vec<Field>) -> TokenStream2 {
    let mut res = quote!();

    let type_name = TokenStream2::from_str(&format!("\"{type_name}\"")).unwrap();

    for field in fields {
        let name = &field.name;

        let field_type = field.field_type();

        let name_string = field.name_as_string();
        let type_string = field.type_as_string();

        let unique = field.unique;
        let secure = field.secure;

        res = quote! {
            #res
            #name: reflected::Field {
                name: #name_string,
                tp: reflected::Type::#field_type,
                type_string: #type_string,
                parent_name: #type_name,
                unique: #unique,
                secure: #secure,
            },
        }
    }

    res
}

fn fields_struct(fields: &Vec<Field>) -> TokenStream2 {
    let mut res = quote!();

    for field in fields {
        let name = &field.name;
        res = quote! {
            #res
            pub #name: reflected::Field,
        }
    }

    quote! {
        #res
    }
}

fn fields_reflect(name: &Ident, fields: &Vec<Field>) -> TokenStream2 {
    let mut res = quote!();

    for field in fields {
        let field_name = &field.name;
        res = quote! {
            #res
            #name::FIELDS.#field_name,
        }
    }

    res
}

fn simple_fields_reflect(name: &Ident, fields: &Vec<Field>) -> TokenStream2 {
    let mut res = quote!();

    for field in fields {
        if !field.is_simple() {
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
        if field.custom() {
            continue;
        }

        let field_name = &field.name;
        let name_string = field.name_as_string();

        res = quote! {
            #res
            #name_string => self.#field_name.to_string(),
        }
    }

    res
}

fn fields_set_value(fields: &Vec<Field>) -> TokenStream2 {
    let mut res = quote!();

    for field in fields {
        if field.custom() {
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

fn parse_fields(fields: &FieldsNamed) -> (Option<String>, Vec<Field>) {
    let mut rename: Option<String> = None;

    let fields: Vec<Field> = fields
        .named
        .iter()
        .map(|field| {
            let name = field.ident.as_ref().unwrap();

            let path = match &field.ty {
                Type::Path(path) => path,
                _ => unreachable!("invalid parse_fields"),
            };

            let tp = &path.path.segments.first().unwrap().ident;

            let attrs: Vec<String> = field
                .attrs
                .iter()
                .map(|a| {
                    let name = get_attribute_name(a);
                    if name == "name" {
                        rename = get_attribute_value(a).expect("name attribute should have value").into();
                    }
                    name
                })
                .collect();

            let unique = attrs.contains(&"unique".to_string());
            let secure = attrs.contains(&"secure".to_string());

            Field {
                name: name.clone(),
                tp: tp.clone(),
                unique,
                secure,
            }
        })
        .collect();

    (rename, fields)
}

fn get_attribute_name(attribute: &Attribute) -> String {
    attribute.path.segments.first().unwrap().ident.to_string()
}

fn get_attribute_value(attribute: &Attribute) -> Option<String> {
    if let Ok(Meta::List(meta_list)) = attribute.parse_meta() {
        if let NestedMeta::Meta(Meta::Path(path)) = &meta_list.nested[0] {
            return Some(path.segments.last()?.ident.to_string());
        }
    }
    None
}
