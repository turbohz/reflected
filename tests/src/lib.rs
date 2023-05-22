#[cfg(test)]
mod test {
    use std::str::FromStr;

    use chrono::{DateTime, Utc};
    use reflected::Reflected;
    use reflected_proc::Reflected;
    use rust_decimal::Decimal;

    #[derive(Reflected, Default, PartialEq, Debug)]
    struct User {
        id:        usize,
        #[unique]
        name:      String,
        #[secure]
        password:  String,
        birthday:  DateTime<Utc>,
        age:       usize,
        custom:    CustomField,
        custom_id: usize,
        cash:      Decimal,
    }

    #[derive(Default, PartialEq, Debug)]
    struct CustomField;

    #[test]
    fn fields() {
        assert!(User::FIELDS.id.is_id());
        assert!(User::FIELDS.name.unique);
        assert!(User::FIELDS.password.secure);
        assert!(User::FIELDS.custom.is_custom());
        assert!(User::FIELDS.custom_id.is_foreign_id());
        assert!(User::FIELDS.birthday.is_date());
        assert!(User::FIELDS.cash.is_decimal());
        assert_eq!(User::fields().len(), 8);
        assert_eq!(User::simple_fields().len(), 5);
    }

    #[test]
    fn get() {
        let birthday = Utc::now();

        let user = User {
            id: 0,
            name: "peter".into(),
            password: "sokol".into(),
            birthday,
            age: 15,
            custom: CustomField,
            custom_id: 0,
            cash: Decimal::from_str("100.25").unwrap(),
        };

        assert_eq!(user.get_value(User::FIELDS.name), "peter".to_string());
        assert_eq!(user.get_value(User::FIELDS.password), "sokol".to_string());
        assert_eq!(user.get_value(User::FIELDS.age), "15".to_string());
        assert_eq!(user.get_value(User::FIELDS.birthday), birthday.to_string());
        assert_eq!(user.get_value(User::FIELDS.cash), "100.25".to_string());
    }

    #[test]
    fn set() {
        let mut user = User {
            id:        0,
            name:      "peter".into(),
            password:  "sokol".into(),
            birthday:  Default::default(),
            age:       15,
            custom:    CustomField,
            custom_id: 0,
            cash:      Default::default(),
        };

        let new_bd = Utc::now();

        user.set_value(User::FIELDS.name, "parker");
        user.set_value(User::FIELDS.password, "soika");
        user.set_value(User::FIELDS.age, "19");
        user.set_value(User::FIELDS.birthday, &new_bd.to_string());
        user.set_value(User::FIELDS.cash, "100.71");

        assert_eq!(user.get_value(User::FIELDS.name), "parker".to_string());
        assert_eq!(user.get_value(User::FIELDS.password), "soika".to_string());
        assert_eq!(user.get_value(User::FIELDS.age), "19".to_string());
        assert_eq!(user.get_value(User::FIELDS.birthday), new_bd.to_string());
        assert_eq!(user.get_value(User::FIELDS.cash), "100.71".to_string());
        assert_eq!(
            user,
            User {
                id:        0,
                name:      "parker".into(),
                password:  "soika".into(),
                birthday:  new_bd,
                age:       19,
                custom:    CustomField,
                custom_id: 0,
                cash:      Decimal::from_str("100.71").unwrap(),
            }
        );
    }

    #[test]
    fn rename() {
        #[derive(Reflected, Default)]
        struct Rename {
            #[name(Renamed_table)]
            id:   usize,
            name: String,
        }

        assert_eq!(Rename::type_name(), "Renamed_table")
    }
}
