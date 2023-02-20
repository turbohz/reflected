use std::{fmt::Debug, str::FromStr};

pub trait TryIntoVal<T: FromStr> {
    fn try_into_val(&self) -> T
    where <T as FromStr>::Err: Debug;
}

impl<T: FromStr> TryIntoVal<T> for &str {
    fn try_into_val(&self) -> T
    where <T as FromStr>::Err: Debug {
        T::from_str(self).unwrap()
    }
}
