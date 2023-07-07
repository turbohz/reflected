use std::str::FromStr;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Fields, FieldsNamed, GenericArgument, Ident, Meta,
    NestedMeta, PathArguments, Type,
    __private::{Span, TokenStream2},
};

use crate::field::Field;

mod field;

/// Data must also derive `Default`
#[proc_macro_derive(Reflected, attributes(unique, name))]
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

    let fields_struct = fields_struct(&name, &fields);
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

            fn fields() -> &'static [&'static reflected::Field<'static, Self>] {
                &[
                    #fields_reflect
                ]
            }

            fn simple_fields() -> &'static [&'static reflected::Field<'static, Self>] {
                &[
                    #simple_fields_reflect
                ]
            }

            fn get_value(&self, field: &'static reflected::Field<'static, Self>) -> String {
                use std::borrow::Borrow;
                use reflected::ToReflectedString;
                let field = field.borrow();

                if field.is_custom() {
                    panic!("get_value method is not supported for custom types: {field:?}");
                }

                match field.name {
                    #fields_get_value
                    _ => unreachable!("Invalid field name in get_value: {}", field.name),
                }
            }

            fn set_value(&mut self, field: &'static reflected::Field<'static, Self>, value: Option<&str>) {
                use reflected::ToReflectedVal;
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

        let unique = field.unique;
        let optional = field.optional;

        let tp = if optional {
            quote! {
                tp: reflected::Type::#field_type.to_optional()
            }
        } else {
            quote! {
                tp: reflected::Type::#field_type
            }
        };

        res = quote! {
            #res
            #name: &reflected::Field {
                name: #name_string,
                #tp,
                parent_name: #type_name,
                unique: #unique,
                optional: #optional,
                _p: std::marker::PhantomData,
            },
        }
    }

    res
}

fn fields_struct(type_name: &Ident, fields: &Vec<Field>) -> TokenStream2 {
    let mut res = quote!();

    for field in fields {
        let name = &field.name;
        res = quote! {
            #res
            pub #name: &'static reflected::Field<'static, #type_name>,
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

        if field.is_bool() {
            if field.optional {
                res = quote! {
                    #res
                    #name_string => self.#field_name.map(|a| if a { "1" } else { "0" }.to_string()).unwrap_or("NULL".to_string()),
                }
            } else {
                res = quote! {
                    #res
                    #name_string => if self.#field_name { "1" } else { "0" }.to_string(),
                }
            }
        } else if field.optional {
            res = quote! {
                #res
                #name_string => self.#field_name.to_reflected_string(),
            }
        } else {
            res = quote! {
                #res
                #name_string => self.#field_name.to_string(),
            }
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

        if field.is_bool() {
            if field.optional {
                res = quote! {
                    #res
                    #name_string =>  {
                        self.#field_name = value.map(|a| match a {
                            "0" => false,
                            "1" => true,
                            _ => unreachable!("Invalid value in bool: {value:?}")
                        })
                    },
                }
            } else {
                res = quote! {
                    #res
                    #name_string =>  {
                        self.#field_name = match value.unwrap() {
                            "0" => false,
                            "1" => true,
                            _ => unreachable!("Invalid value in bool: {value:?}")
                        }
                    },
                }
            }
        } else if field.optional {
            res = quote! {
                #res
                #name_string => self.#field_name = value.map(|a| a.to_reflected_val()
                    .expect(&format!("Failed to convert to: {} from: {}", #name_string, a))),
            }
        } else {
            res = quote! {
                #res
                #name_string => self.#field_name = value.unwrap().to_reflected_val()
                .expect(&format!("Failed to convert to: {} from: {}", #name_string, value.unwrap())),
            }
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
            let name = field.ident.as_ref().unwrap().clone();
            let mut optional = false;

            let path = match &field.ty {
                Type::Path(path) => path,
                _ => unreachable!("invalid parse_fields"),
            };

            let mut tp = path.path.segments.first().unwrap().ident.clone();

            //            dbg!(&tp);

            if tp == "Option" {
                optional = true;
                let args = &path.path.segments.first().unwrap().arguments;
                if let PathArguments::AngleBracketed(args) = args {
                    if let GenericArgument::Type(generic_tp) = args.args.first().unwrap() {
                        let ident = generic_tp.to_token_stream().to_string();
                        let ident = Ident::new(&ident, Span::call_site());
                        //                        dbg!(&ident);
                        //name = ident.clone();
                        tp = ident;
                    } else {
                        unreachable!()
                    }
                } else {
                    unreachable!()
                }
            }

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

            Field {
                name,
                tp,
                unique,
                optional,
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
