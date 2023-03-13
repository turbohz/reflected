use crate::Type;

#[derive(Debug)]
pub struct Field {
    pub name:        &'static str,
    pub tp:          Type,
    pub type_string: &'static str,
    pub parent_name: &'static str,
    pub unique:      bool,
    pub secure:      bool,
}

impl Field {
    pub fn is_id(&self) -> bool {
        self.name == "id"
    }

    pub fn is_foreign_id(&self) -> bool {
        self.name.contains("_id")
    }

    pub fn is_custom(&self) -> bool {
        matches!(self.tp, Type::Custom)
    }

    pub fn is_simple(&self) -> bool {
        !self.is_id() && !self.is_custom() && !self.is_foreign_id()
    }

    pub fn is_text(&self) -> bool {
        matches!(self.tp, Type::Text)
    }

    pub fn is_number(&self) -> bool {
        matches!(self.tp, Type::Integer | Type::Float)
    }
}
