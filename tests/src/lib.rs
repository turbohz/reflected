use chrono::NaiveDateTime;
use reflected::Reflected;
use rust_decimal::Decimal;

#[derive(Default, Clone, PartialEq, Debug)]
struct CustomField;

#[derive(Reflected, Clone, Default, PartialEq, Debug)]
pub struct User {
    id:   usize,
    name: String,

    birthday:  NaiveDateTime,
    age:       usize,
    custom:    CustomField,
    custom_id: usize,
    cash:      Decimal,
    is_poros:  bool,
    height:    f64,

    str_opt:     Option<std::string::String>,
    usize_opt:   Option<usize>,
    bool_opt:    Option<bool>,
    decimal_opt: Option<Decimal>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Type {
    Bool,
    Int,
    Text,
}

impl Default for Type {
    fn default() -> Self {
        Self::Int
    }
}

// Struct with raw name and identifiers
#[derive(Reflected, Clone, Default, PartialEq, Debug)]
pub struct r#FieldSchema {
    pub id: usize,
    pub name: std::string::String,
    pub r#type: Type,
    pub description: std::option::Option<std::string::String>,
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use chrono::{NaiveDateTime, Utc};
    use reflected::{Reflected, ReflectedEq};
    use rust_decimal::Decimal;

    use crate::{CustomField, User, Type, FieldSchema};

    #[test]
    fn convert_date() {
        let date = Utc::now().naive_utc();

        dbg!(&date);

        let date_string = date.to_string();

        dbg!(&date_string);

        let parsed_date = NaiveDateTime::parse_from_str(&date_string, "%Y-%m-%d %H:%M:%S%.9f");

        dbg!(&parsed_date);
    }

    #[test]
    fn test_raw_name_and_fields() {
        assert!(FieldSchema::FIELDS.id.is_id());
        assert!(FieldSchema::FIELDS.name.is_text());
        assert!(FieldSchema::FIELDS.r#type.is_custom());
        assert!(FieldSchema::FIELDS.description.is_optional());
        assert!(FieldSchema::FIELDS.description.is_text());

        assert_eq!(FieldSchema::FIELDS.id.type_name, "usize");
        assert_eq!(FieldSchema::FIELDS.name.type_name, "String");
        assert_eq!(FieldSchema::FIELDS.r#type.type_name, "Type");
        assert_eq!(FieldSchema::FIELDS.description.type_name, "String");

        let record_schema = FieldSchema {
            id: 436,
            r#type: Type::Bool,
            name: "Active".to_string(),
            description: None,
        };

        assert_eq!(record_schema.get_value(FieldSchema::FIELDS.id), "436".to_string());
        assert_eq!(record_schema.get_value(FieldSchema::FIELDS.name), "Active".to_string());
        assert_eq!(record_schema.get_value(FieldSchema::FIELDS.description), "NULL".to_string());
    }

    #[test]
    fn fields() {
        assert!(User::FIELDS.id.is_id());
        assert!(User::FIELDS.custom.is_custom());
        assert!(User::FIELDS.custom_id.is_foreign_id());
        assert!(User::FIELDS.birthday.is_date());
        assert!(User::FIELDS.cash.is_decimal());
        assert!(User::FIELDS.is_poros.is_bool());
        assert!(User::FIELDS.height.is_float());

        assert!(User::FIELDS.str_opt.is_optional());
        assert!(User::FIELDS.str_opt.is_text());

        assert!(User::FIELDS.usize_opt.is_optional());
        assert!(User::FIELDS.usize_opt.is_integer());

        assert!(User::FIELDS.bool_opt.is_optional());
        assert!(User::FIELDS.bool_opt.is_bool());

        assert!(User::FIELDS.decimal_opt.is_optional());
        assert!(User::FIELDS.decimal_opt.is_decimal());

        assert_eq!(User::fields().len(), 13);
        assert_eq!(User::simple_fields().len(), 10);
    }

    #[test]
    fn types() {
        assert_eq!(User::FIELDS.id.type_name, "usize");
        assert_eq!(User::FIELDS.custom.type_name, "CustomField");
        assert_eq!(User::FIELDS.birthday.type_name, "NaiveDateTime");
        assert_eq!(User::FIELDS.cash.type_name, "Decimal");
        assert_eq!(User::FIELDS.is_poros.type_name, "bool");
        assert_eq!(User::FIELDS.height.type_name, "f64");
        assert_eq!(User::FIELDS.str_opt.type_name, "String");
        assert_eq!(User::FIELDS.usize_opt.type_name, "usize");
        assert_eq!(User::FIELDS.bool_opt.type_name, "bool");
        assert_eq!(User::FIELDS.decimal_opt.type_name, "Decimal");
    }

    #[test]
    fn get() {
        let birthday = Utc::now().naive_utc();

        let mut user = User {
            id: 0,
            name: "peter".into(),
            birthday,
            age: 15,
            custom: CustomField,
            custom_id: 0,
            cash: Decimal::from_str("100.25").unwrap(),
            is_poros: false,
            height: 6.45,
            str_opt: None,
            usize_opt: None,
            bool_opt: None,
            decimal_opt: None,
        };

        assert_eq!(user.get_value(User::FIELDS.name), "peter".to_string());
        assert_eq!(user.get_value(User::FIELDS.age), "15".to_string());
        assert_eq!(user.get_value(User::FIELDS.birthday), birthday.to_string());
        assert_eq!(user.get_value(User::FIELDS.cash), "100.25".to_string());
        assert_eq!(user.get_value(User::FIELDS.is_poros), "0".to_string());
        assert_eq!(user.get_value(User::FIELDS.height), "6.45".to_string());

        assert_eq!(user.get_value(User::FIELDS.str_opt), "NULL".to_string());
        assert_eq!(user.get_value(User::FIELDS.usize_opt), "NULL".to_string());
        assert_eq!(user.get_value(User::FIELDS.bool_opt), "NULL".to_string());
        assert_eq!(user.get_value(User::FIELDS.decimal_opt), "NULL".to_string());

        user.str_opt = Some("stre".to_string());
        user.usize_opt = Some(222);
        user.bool_opt = Some(false);
        user.decimal_opt = Some(Decimal::from_str("100.25").unwrap());

        assert_eq!(user.get_value(User::FIELDS.str_opt), "stre".to_string());
        assert_eq!(user.get_value(User::FIELDS.usize_opt), "222".to_string());
        assert_eq!(user.get_value(User::FIELDS.bool_opt), "0".to_string());
        assert_eq!(user.get_value(User::FIELDS.decimal_opt), "100.25".to_string());
    }

    #[test]
    fn set() {
        let mut user = User {
            id:          0,
            name:        "peter".into(),
            birthday:    Default::default(),
            age:         15,
            custom:      CustomField,
            custom_id:   0,
            cash:        Default::default(),
            is_poros:    false,
            height:      6.45,
            str_opt:     None,
            usize_opt:   None,
            bool_opt:    None,
            decimal_opt: None,
        };

        let new_bd = Utc::now().naive_utc();

        user.set_value(User::FIELDS.name, "parker".into());
        user.set_value(User::FIELDS.age, "19".into());
        user.set_value(User::FIELDS.birthday, Some(&new_bd.to_string()));
        user.set_value(User::FIELDS.cash, "100.71".into());
        user.set_value(User::FIELDS.is_poros, "1".into());
        user.set_value(User::FIELDS.height, "5.467".into());

        assert_eq!(user.get_value(User::FIELDS.name), "parker".to_string());
        assert_eq!(user.get_value(User::FIELDS.age), "19".to_string());
        assert_eq!(user.get_value(User::FIELDS.birthday), new_bd.to_string());
        assert_eq!(user.get_value(User::FIELDS.cash), "100.71".to_string());
        assert_eq!(user.get_value(User::FIELDS.is_poros), "1".to_string());
        assert_eq!(user.get_value(User::FIELDS.height), "5.467".to_string());

        user.set_value(User::FIELDS.str_opt, "sokol".into());
        user.set_value(User::FIELDS.usize_opt, "555".into());
        user.set_value(User::FIELDS.bool_opt, "1".into());
        user.set_value(User::FIELDS.decimal_opt, "100.71".into());

        assert_eq!(user.get_value(User::FIELDS.str_opt), "sokol".to_string());
        assert_eq!(user.get_value(User::FIELDS.usize_opt), "555".to_string());
        assert_eq!(user.get_value(User::FIELDS.bool_opt), "1".to_string());
        assert_eq!(user.get_value(User::FIELDS.decimal_opt), "100.71".to_string());

        user.set_value(User::FIELDS.str_opt, None);
        user.set_value(User::FIELDS.usize_opt, None);
        user.set_value(User::FIELDS.bool_opt, None);
        user.set_value(User::FIELDS.decimal_opt, None);

        assert_eq!(user.get_value(User::FIELDS.str_opt), "NULL".to_string());
        assert_eq!(user.get_value(User::FIELDS.usize_opt), "NULL".to_string());
        assert_eq!(user.get_value(User::FIELDS.bool_opt), "NULL".to_string());
        assert_eq!(user.get_value(User::FIELDS.decimal_opt), "NULL".to_string());

        assert_eq!(
            user,
            User {
                id:          0,
                name:        "parker".into(),
                birthday:    new_bd,
                age:         19,
                custom:      CustomField,
                custom_id:   0,
                cash:        Decimal::from_str("100.71").unwrap(),
                is_poros:    true,
                height:      5.467,
                str_opt:     None,
                usize_opt:   None,
                bool_opt:    None,
                decimal_opt: None,
            }
        );
    }

    #[test]
    fn random() {
        let _user = User::random();
        dbg!(_user);
    }

    #[test]
    fn reflected_eq() {
        #[derive(Default, Reflected, Clone)]
        struct Test {
            id:   usize,
            name: String,

            birthday:  NaiveDateTime,
            age:       usize,
            custom_id: usize,
            cash:      Decimal,
            is_poros:  bool,
            height:    f64,
        }

        let user_1 = Test::random();
        let mut user_2 = user_1.clone();

        user_1.assert_eq(&user_2);

        user_2.height += 0.0001;

        user_1.assert_eq(&user_2);
    }

    #[test]
    fn get_float() {
        #[derive(Default, Reflected)]
        struct Data {
            float32: f32,
            float64: f64,
        }

        let mut data = Data {
            float32: 5.0,
            float64: 1.0,
        };

        assert_eq!(data.get_value(Data::FIELDS.float32), "5.0");
        assert_eq!(data.get_value(Data::FIELDS.float64), "1.0");

        data.float32 = 0.42332;
        data.float64 = 0.438297489;

        assert_eq!(data.get_value(Data::FIELDS.float32), "0.42332");
        assert_eq!(data.get_value(Data::FIELDS.float64), "0.438297489");
    }
}
