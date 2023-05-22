#[cfg(test)]
mod test {
    use chrono::{DateTime, Utc};
    use reflected::Reflected;
    use reflected_proc::Reflected;

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
        assert_eq!(User::fields().len(), 7);
        assert_eq!(User::simple_fields().len(), 4);
    }

    #[test]
    fn get() {
        let user = User {
            id:        0,
            name:      "peter".into(),
            password:  "sokol".into(),
            birthday:  Default::default(),
            age:       15,
            custom:    CustomField,
            custom_id: 0,
        };

        assert_eq!(user.get_value(&User::FIELDS.name), "peter".to_string());
        assert_eq!(user.get_value(&User::FIELDS.password), "sokol".to_string());
        assert_eq!(user.get_value(&User::FIELDS.age), "15".to_string());
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
        };

        user.set_value(&User::FIELDS.name, "parker");
        user.set_value(&User::FIELDS.password, "soika");
        user.set_value(&User::FIELDS.age, "19");

        assert_eq!(user.get_value(&User::FIELDS.name), "parker".to_string());
        assert_eq!(user.get_value(&User::FIELDS.password), "soika".to_string());
        assert_eq!(user.get_value(&User::FIELDS.age), "19".to_string());
        assert_eq!(
            user,
            User {
                id:        0,
                name:      "parker".into(),
                password:  "soika".into(),
                birthday:  Default::default(),
                age:       19,
                custom:    CustomField,
                custom_id: 0,
            }
        );
    }

    #[test]
    fn date() {
        let birthday = Utc::now();

        let bd_string = birthday.to_string();

        let mut user = User {
            id: 0,
            name: "peter".into(),
            password: "sokol".into(),
            birthday,
            age: 15,
            custom: CustomField,
            custom_id: 0,
        };

        assert_eq!(bd_string, user.get_value(User::FIELDS.birthday));

        let new_bd = Utc::now();
        let new_bd_string = new_bd.to_string();

        user.set_value(User::FIELDS.birthday, &new_bd_string);

        assert_eq!(new_bd, user.birthday);
    }
}
