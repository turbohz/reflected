#[cfg(test)]
mod test {
    use reflected::Reflected;
    use reflected_proc::Reflected;

    #[derive(Reflected, Default, PartialEq, Debug)]
    struct User {
        #[unique]
        name: String,
        age: usize,
    }

    #[test]
    fn fields() {
        assert!(User::FIELDS.name.unique);
        assert_eq!(User::fields().len(), 2);
    }

    #[test]
    fn get() {
        let user = User {
            name: "peter".into(),
            age: 15,
        };

        assert_eq!(user.get_value(&User::FIELDS.name), "peter".to_string());
        assert_eq!(user.get_value(&User::FIELDS.age), "15".to_string());
    }

    #[test]
    fn set() {
        let mut user = User {
            name: "peter".into(),
            age: 15,
        };

        user.set_value(&User::FIELDS.name, "parker");
        user.set_value(&User::FIELDS.age, "19");

        assert_eq!(user.get_value(&User::FIELDS.name), "parker".to_string());
        assert_eq!(user.get_value(&User::FIELDS.age), "19".to_string());
        assert_eq!(
            user,
            User {
                name: "parker".into(),
                age: 19
            }
        );
    }
}
