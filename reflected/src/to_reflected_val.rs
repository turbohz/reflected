use std::{fmt::Debug, str::FromStr};

pub trait ToReflectedVal<T: FromStr> {
    fn to_reflected_val(&self) -> T
    where <T as FromStr>::Err: Debug;
}

impl<T: FromStr> ToReflectedVal<T> for &str {
    fn to_reflected_val(&self) -> T
    where <T as FromStr>::Err: Debug {
        T::from_str(self).unwrap()
    }
}
