pub mod field;
pub mod field_type;
pub mod try_into_val;

use std::borrow::Borrow;

use chrono::Utc;
pub use field::*;
pub use field_type::*;
use rand::{
    distributions::{Alphanumeric, DistString},
    random, thread_rng, Rng,
};
use rust_decimal::Decimal;
pub use try_into_val::*;

pub trait Reflected: Default {
    fn type_name() -> &'static str;

    fn fields() -> &'static [Field];
    fn simple_fields() -> &'static [Field];

    fn get_value(&self, field: impl Borrow<Field>) -> String;
    fn set_value(&mut self, field: impl Borrow<Field>, value: &str);

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
                res.set_value(field, &str);
            } else if field.is_number() {
                let val: u32 = rng.gen_range(0..100);
                let val = val.to_string();
                res.set_value(field, &val);
            } else if field.is_date() {
                res.set_value(field, &Utc::now().to_string());
            } else if field.is_decimal() {
                let dec = Decimal::new(random(), rng.gen_range(0..28));
                res.set_value(field, &dec.to_string());
            };
        }

        res
    }
}
