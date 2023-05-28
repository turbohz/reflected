pub mod field;
pub mod field_type;
pub mod to_reflected_string;
pub mod to_reflected_val;

use chrono::Utc;
pub use field::*;
pub use field_type::*;
use rand::{
    distributions::{Alphanumeric, DistString},
    random, thread_rng, Rng,
};
use rust_decimal::Decimal;
pub use to_reflected_string::*;
pub use to_reflected_val::*;

pub trait Reflected: Default {
    fn type_name() -> &'static str;

    fn fields() -> &'static [&'static Field];
    fn simple_fields() -> &'static [&'static Field];

    fn get_value(&self, field: &'static Field) -> String;
    fn set_value(&mut self, field: &'static Field, value: Option<&str>);

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
                res.set_value(field, Some(&str));
            } else if field.is_number() {
                let val: u32 = rng.gen_range(0..100);
                let val = val.to_string();
                res.set_value(field, Some(&val));
            } else if field.is_date() {
                res.set_value(field, Some(&Utc::now().to_string()));
            } else if field.is_decimal() {
                let dec = Decimal::new(random(), rng.gen_range(0..28));
                res.set_value(field, Some(&dec.to_string()));
            };
        }

        res
    }
}
