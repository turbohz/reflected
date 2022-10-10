use crate::Type;

#[derive(Debug)]
pub struct Field {
    pub name: &'static str,
    pub tp: Type,
}

impl Field {
    pub fn is_id(&self) -> bool {
        self.name == "id"
    }

    pub fn is_unsupported(&self) -> bool {
        matches!(self.tp, Type::Unsupported)
    }

    pub fn is_text(&self) -> bool {
        matches!(self.tp, Type::Text)
    }

    pub fn is_number(&self) -> bool {
        matches!(self.tp, Type::Integer | Type::Float)
    }
}
