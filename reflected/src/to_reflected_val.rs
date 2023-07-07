use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

pub trait ToReflectedVal<T: FromStr> {
    fn to_reflected_val(&self) -> Result<T, String>
    where <T as FromStr>::Err: Debug + Display;
}

impl<T: FromStr> ToReflectedVal<T> for &str {
    fn to_reflected_val(&self) -> Result<T, String>
    where <T as FromStr>::Err: Debug + Display {
        T::from_str(self).map_err(|e| e.to_string())
    }
}
