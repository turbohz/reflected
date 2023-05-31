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

pub trait Reflected: Default + 'static {
    fn type_name() -> &'static str;

    fn fields() -> &'static [&'static Field<'static, Self>];
    fn simple_fields() -> &'static [&'static Field<'static, Self>];

    fn get_value(&self, field: &'static Field<'static, Self>) -> String;
    fn set_value(&mut self, field: &'static Field<'static, Self>, value: Option<&str>);

    fn field_by_name(name: &str) -> &'static Field<'static, Self> {
        Self::fields().iter().find(|a| a.name == name).unwrap()
    }

    fn value_by_name(&self, name: &str) -> String {
        self.get_value(Self::field_by_name(name))
    }

    fn random() -> Self {
        let mut res = Self::default();

        for field in Self::simple_fields() {
            res.set_value(field, random_val(&field.tp).as_deref());
        }

        res
    }
}

fn random_val(tp: &Type) -> Option<String> {
    let mut rng = thread_rng();

    match tp {
        Type::Text => Alphanumeric.sample_string(&mut rng, 8).into(),
        Type::Integer | Type::Float => rng.gen_range(0..100).to_string().into(),
        Type::Date => Utc::now().to_string().into(),
        Type::Decimal => Decimal::new(random(), rng.gen_range(0..28)).to_string().into(),
        Type::Bool => rng.gen_range(0..1).to_string().into(),
        Type::Optional(opt) => {
            if rng.gen() {
                random_val(&opt.to_type())
            } else {
                None
            }
        }
        _ => unreachable!("Failed to gen random value for: {tp:?}"),
    }
}
