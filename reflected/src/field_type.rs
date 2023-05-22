#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Float,
    Integer,
    Text,
    Date,
    Decimal,
    Custom,
}
