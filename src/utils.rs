extern crate core;

#[macro_export]
macro_rules! to_u128 {
    ($($x:expr),+) => (
        {($($x.parse::<u128>().unwrap_or(0)),+)}
    );
}

#[macro_export]
macro_rules! to_u32 {
    ($($x:expr),+) => (
        {($($x.parse::<u32>().unwrap_or(0)),+)}
    );
}

#[macro_export]
macro_rules! parse_into2 {
    ($x:ty, $y:expr, $e:expr) => {{
        let r = if let Some(x) = $y.parse::<$x>().ok() {
            x
        } else {
            return $e;
        };
        r
    }};
}

pub fn error() -> String {
    "-1".to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn conversion() {
        assert_eq!(crate::to_u128!("128"), 128_u128);
        assert_eq!(crate::to_u128!("invalid"), 0_u128);
    }
}
