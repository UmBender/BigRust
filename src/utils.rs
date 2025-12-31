use crate::BigInt;
use crate::Sign;
impl From<u8> for BigInt {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::new(),
            n => Self::from_u128(n as u128, Sign::Positive),
        }
    }
}

impl From<u16> for BigInt {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::new(),
            n => Self::from_u128(n as u128, Sign::Positive),
        }
    }
}

impl From<u32> for BigInt {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::new(),
            n => Self::from_u128(n as u128, Sign::Positive),
        }
    }
}

impl From<u64> for BigInt {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::new(),
            n => Self::from_u128(n as u128, Sign::Positive),
        }
    }
}

impl From<u128> for BigInt {
    fn from(value: u128) -> Self {
        match value {
            0 => Self::new(),
            n => Self::from_u128(n, Sign::Positive),
        }
    }
}

impl From<usize> for BigInt {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::new(),
            n => Self::from_u128(n as u128, Sign::Positive),
        }
    }
}

impl From<i8> for BigInt {
    fn from(value: i8) -> Self {
        match value {
            0 => Self::new(),
            n if n < 0 => Self::from_u128((-n) as u128, Sign::Negative),
            n => Self::from_u128(n as u128, Sign::Positive),
        }
    }
}

impl From<i16> for BigInt {
    fn from(value: i16) -> Self {
        match value {
            0 => Self::new(),
            n if n < 0 => Self::from_u128((-n) as u128, Sign::Negative),
            n => Self::from_u128(n as u128, Sign::Positive),
        }
    }
}

impl From<i32> for BigInt {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::new(),
            n if n < 0 => Self::from_u128((-n) as u128, Sign::Negative),
            n => Self::from_u128(n as u128, Sign::Positive),
        }
    }
}

impl From<i64> for BigInt {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::new(),
            n if n < 0 => Self::from_u128((-n) as u128, Sign::Negative),
            n => Self::from_u128(n as u128, Sign::Positive),
        }
    }
}

impl From<i128> for BigInt {
    fn from(value: i128) -> Self {
        match value {
            0 => Self::new(),
            n if n < 0 => Self::from_u128((-n) as u128, Sign::Negative),
            n => Self::from_u128(n as u128, Sign::Positive),
        }
    }
}

impl From<isize> for BigInt {
    fn from(value: isize) -> Self {
        match value {
            0 => Self::new(),
            n if n < 0 => Self::from_u128((-n) as u128, Sign::Negative),
            n => Self::from_u128(n as u128, Sign::Positive),
        }
    }
}

impl From<&str> for BigInt {
    fn from(value: &str) -> Self {
        Self::from_slice_str(value)
    }
}

impl From<&BigInt> for String {
    fn from(value: &BigInt) -> Self {
        value.to_string()
    }
}

impl From<BigInt> for String {
    fn from(value: BigInt) -> Self {
        value.to_string()
    }
}
