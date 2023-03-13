#[cfg(test)]
mod test {
    use reflected::Reflected;
    use reflected_proc::Reflected;

    #[derive(Reflected, Default, PartialEq, Debug)]
    struct User {
        id:       usize,
        #[unique]
        name:     String,
        #[secure]
        password: String,
        age:      usize,
        custom:   CustomField,
    }

    #[derive(Default, PartialEq, Debug)]
    struct CustomField;

    #[test]
    fn fields() {
        assert!(User::FIELDS.id.is_id());
        assert!(User::FIELDS.name.unique);
        assert!(User::FIELDS.password.secure);
        assert!(User::FIELDS.custom.is_custom());
        assert_eq!(User::fields().len(), 5);
        assert_eq!(User::simple_fields().len(), 3);
        dbg!(User::simple_fields());
    }

    #[test]
    fn get() {
        let user = User {
            id:       0,
            name:     "peter".into(),
            password: "sokol".into(),
            age:      15,
            custom:   CustomField,
        };

        assert_eq!(user.get_value(&User::FIELDS.name), "peter".to_string());
        assert_eq!(user.get_value(&User::FIELDS.password), "sokol".to_string());
        assert_eq!(user.get_value(&User::FIELDS.age), "15".to_string());
    }

    #[test]
    fn set() {
        let mut user = User {
            id:       0,
            name:     "peter".into(),
            password: "sokol".into(),
            age:      15,
            custom:   CustomField,
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
                id:       0,
                name:     "parker".into(),
                password: "soika".into(),
                age:      19,
                custom:   CustomField,
            }
        );
    }
}
