use std::ops::Deref;

use crate::Type;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Field<'a> {
    pub name:        &'a str,
    pub tp:          Type,
    pub parent_name: &'a str,
    pub unique:      bool,
    pub optional:    bool,
}

impl Field<'_> {
    pub fn is_id(&self) -> bool {
        self.name == "id"
    }

    pub fn is_foreign_id(&self) -> bool {
        self.name.contains("_id")
    }

    pub fn is_simple(&self) -> bool {
        !self.is_id() && !self.is_custom() && !self.is_foreign_id()
    }

    pub fn is_secure(&self) -> bool {
        self.name == "password"
    }
}

impl Deref for Field<'_> {
    type Target = Type;
    fn deref(&self) -> &Self::Target {
        &self.tp
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{Field, Type};

    #[test]
    fn store_in_map() {
        let field: &'static Field = &Field {
            name:        "",
            tp:          Type::Float,
            parent_name: "",
            unique:      false,
            optional:    false,
        };

        let mut map = HashMap::<&'static Field, String>::default();
        map.insert(field, Default::default());
    }
}
