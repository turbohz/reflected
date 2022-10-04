
#[derive(Debug, Eq, PartialEq)]
pub enum Type {
    Float,
    Integer,
    Text,
}

#[derive(Debug)]
pub struct Field {
    pub name:   &'static str,
    pub tp:     Type,
    pub unique: bool,
}

impl Field {
    pub fn is_id(&self) -> bool {
        self.name == "rowid" || self.name == "id"
    }

    pub fn is_text(&self) -> bool {
        matches!(self.tp, Type::Text)
    }

    pub fn is_number(&self) -> bool {
        matches!(self.tp, Type::Integer | Type::Float)
    }
}
