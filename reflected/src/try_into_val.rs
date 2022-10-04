use std::str::FromStr;

pub trait TryIntoVal<T> {
    fn try_into_val(&self) -> T;
}

impl TryIntoVal<i32> for &str {
    fn try_into_val(&self) -> i32 {
        i32::from_str(self).unwrap()
    }
}

impl TryIntoVal<u64> for &str {
    fn try_into_val(&self) -> u64 {
        u64::from_str(self).unwrap()
    }
}

impl TryIntoVal<Option<u64>> for &str {
    fn try_into_val(&self) -> Option<u64> {
        u64::from_str(self).ok()
    }
}

impl TryIntoVal<String> for &str {
    fn try_into_val(&self) -> String {
        self.to_string()
    }
}
