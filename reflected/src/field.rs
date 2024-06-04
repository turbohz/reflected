use std::{
    any::type_name,
    fmt::{Debug, Formatter},
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::Deref,
};

use crate::Type;

pub type FieldRef<T> = &'static Field<'static, T>;

pub struct Field<'a, T> {
    pub name:        &'a str,
    pub tp:          Type,
    pub type_name:   &'a str,
    pub parent_name: &'a str,
    pub optional:    bool,
    pub _p:          PhantomData<T>,
}

impl<T> Field<'_, T> {
    pub fn is_id(&self) -> bool {
        self.name == "id"
    }

    pub fn is_foreign_id(&self) -> bool {
        self.name.contains("_id")
    }

    pub fn is_simple(&self) -> bool {
        !self.is_id() && !self.is_custom() && !self.is_foreign_id()
    }
}

impl<'a, T> Debug for Field<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Field {{ name: {}, tp: {:?}, parent_name: {}, optional: {} }}",
            self.name, self.tp, self.parent_name, self.optional
        )
    }
}

impl<T> Deref for Field<'_, T> {
    type Target = Type;
    fn deref(&self) -> &Self::Target {
        &self.tp
    }
}

impl<'a, T> Hash for Field<'a, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.tp.hash(state);
        self.parent_name.hash(state);
        self.optional.hash(state);
        type_name::<T>().hash(state);
    }
}

impl<'a, T> PartialEq for Field<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(other.name)
            && self.tp.eq(&other.tp)
            && self.parent_name.eq(other.parent_name)
            && self.optional.eq(&other.optional)
    }
}

impl<'a, T> Eq for Field<'a, T> {}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, marker::PhantomData};

    use crate::{Field, Type};

    struct Strekta {}

    #[test]
    fn store_in_map() {
        let field: &'static Field<Strekta> = &Field {
            name:        "",
            tp:          Type::Float,
            type_name:   "",
            parent_name: "",
            optional:    false,
            _p:          PhantomData,
        };

        let mut map = HashMap::<&'static Field<Strekta>, String>::default();
        map.insert(field, Default::default());
    }

    #[test]
    fn debug() {
        let field: &'static Field<Strekta> = &Field {
            name:        "Name",
            tp:          Type::Float,
            type_name:   "f32",
            parent_name: "SomeStruct",
            optional:    false,
            _p:          PhantomData,
        };

        dbg!(field);
    }
}
