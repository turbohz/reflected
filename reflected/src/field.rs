#[derive(Debug, Eq, PartialEq)]
pub enum Type {
    Float,
    Integer,
    Text,
}

#[derive(Debug)]
pub struct Field {
    pub name: &'static str,
    pub tp: Type,
}

impl Field {
    pub fn is_text(&self) -> bool {
        matches!(self.tp, Type::Text)
    }

    pub fn is_number(&self) -> bool {
        matches!(self.tp, Type::Integer | Type::Float)
    }
}
