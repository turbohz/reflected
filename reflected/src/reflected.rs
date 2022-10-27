use std::borrow::Borrow;

use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng, Rng,
};

use crate::Field;

pub trait Reflected: Default {
    fn type_name() -> &'static str;

    fn fields() -> &'static [Field];
    fn subfields(field: impl Borrow<Field>) -> &'static [Field];

    fn get_value(&self, field: impl Borrow<Field>) -> String;
    fn set_value(&mut self, value: &str, field: impl Borrow<Field>);

    fn field_by_name(name: &str) -> &'static Field {
        Self::fields().iter().find(|a| a.name == name).unwrap()
    }

    fn value_by_name(&self, name: &str) -> String {
        self.get_value(Self::field_by_name(name))
    }

    fn random() -> Self {
        let mut res = Self::default();

        let mut rng = thread_rng();

        for field in Self::fields() {
            if field.is_text() {
                let str = Alphanumeric.sample_string(&mut rng, 8);
                res.set_value(&str, field);
            } else if field.is_number() {
                let val: u32 = rng.gen_range(0..100);
                let val = val.to_string();
                res.set_value(&val, field);
            };
        }

        res
    }
}

/// Review or rename this
pub fn to_database_string<Val: ToString + ?Sized>(val: &Val, is_text: bool) -> String {
    if is_text {
        format!("\'{}\'", val.to_string())
    } else {
        val.to_string()
    }
}
