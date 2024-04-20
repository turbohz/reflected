use crate::{random::random_val, Field};

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

        for field in Self::fields() {
            if field.is_custom() {
                continue;
            }
            res.set_value(field, random_val(&field.tp).as_deref());
        }

        res
    }
}
