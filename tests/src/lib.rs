#[cfg(test)]
mod test {
    use reflected::Reflected;
    use reflected_proc::Reflected;

    #[derive(Reflected, Default, PartialEq, Debug)]
    struct User {
        #[unique]
        name: String,
        #[secure]
        password: String,
        age: usize,
    }

    #[test]
    fn fields() {
        assert!(User::FIELDS.name.unique);
        assert!(User::FIELDS.password.secure);
        assert_eq!(User::fields().len(), 3);
    }

    #[test]
    fn get() {
        let user = User {
            name: "peter".into(),
            password: "sokol".into(),
            age: 15,
        };

        assert_eq!(user.get_value(&User::FIELDS.name), "peter".to_string());
        assert_eq!(user.get_value(&User::FIELDS.password), "sokol".to_string());
        assert_eq!(user.get_value(&User::FIELDS.age), "15".to_string());
    }

    #[test]
    fn set() {
        let mut user = User {
            name: "peter".into(),
            password: "sokol".into(),
            age: 15,
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
                name: "parker".into(),
                password: "soika".into(),
                age: 19
            }
        );
    }
}
