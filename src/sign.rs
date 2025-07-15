use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sign {
    Positive,
    Negative,
    Zero,
}

impl PartialOrd for Sign {
    fn lt(&self, other: &Self) -> bool {
        // <
        if *self == *other {
            return false;
        }
        if *self == Self::Negative {
            return true;
        }
        if *self == Self::Zero && *other == Self::Positive {
            return true;
        }
        false
    }

    fn le(&self, other: &Self) -> bool {
        // <=
        if *self == *other {
            return true;
        }
        if *self == Self::Negative {
            return true;
        }
        if *self == Self::Zero && *other == Self::Positive {
            return true;
        }
        false
    }

    fn gt(&self, other: &Self) -> bool {
        // >
        if *self == *other {
            return false;
        }
        if Self::Negative == *other {
            return true;
        }
        if *self == Self::Positive && Self::Zero == *other {
            return true;
        }
        false
    }

    fn ge(&self, other: &Self) -> bool {
        // >=
        if *self == *other {
            return true;
        }
        if Self::Negative == *other {
            return true;
        }
        if *self == Self::Positive && Self::Zero == *other {
            return true;
        }
        false
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if *self == *other {
            return Some(Ordering::Equal);
        }
        if *self < *other {
            return Some(Ordering::Less);
        }
        if *self > *other {
            return Some(Ordering::Greater);
        }
        None
    }
}

impl Sign {
    pub fn reverse(&self) -> Self {
        match *self {
            Self::Zero => Self::Zero,
            Self::Positive => Self::Negative,
            Self::Negative => Self::Positive,
        }
    }
}
