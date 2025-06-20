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

/// Parses a string in the format "numerator/denominator" into a tuple of u128 values.
/// Returns None if the string is not in the correct format or if parsing fails.
pub fn parse_ratio(ratio_str: &str) -> Option<(u128, u128)> {
    let parts: Vec<&str> = ratio_str.split('/').collect();
    if parts.len() != 2 {
        return None;
    }

    let numerator = parts[0].parse::<u128>().ok()?;
    let denominator = parts[1].parse::<u128>().ok()?;

    Some((numerator, denominator))
}

pub fn error() -> String {
    "-1".to_string()
}

#[cfg(test)]
mod tests {
    use crate::utils::parse_ratio;

    #[test]
    fn conversion() {
        assert_eq!(crate::to_u128!("128"), 128_u128);
        assert_eq!(crate::to_u128!("invalid"), 0_u128);
    }

    #[test]
    fn test_ratio_parser() {
        assert_eq!(parse_ratio("1/2"), Some((1, 2)));
        assert_eq!(parse_ratio("1234/56789"), Some((1234, 56789)));
    }
}
