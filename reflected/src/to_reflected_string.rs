use rust_decimal::Decimal;

pub trait ToReflectedString {
    fn to_reflected_string(&self) -> String;
}

impl ToReflectedString for Option<&str> {
    fn to_reflected_string(&self) -> String {
        self.unwrap_or("NULL").to_string()
    }
}

impl ToReflectedString for Option<String> {
    fn to_reflected_string(&self) -> String {
        self.clone().unwrap_or("NULL".to_string())
    }
}

impl ToReflectedString for Option<usize> {
    fn to_reflected_string(&self) -> String {
        self.map(|a| a.to_string()).unwrap_or("NULL".to_string())
    }
}

impl ToReflectedString for Option<Decimal> {
    fn to_reflected_string(&self) -> String {
        self.map(|a| a.to_string()).unwrap_or("NULL".to_string())
    }
}
