use crate::Ordering;
use crate::Sign;
use std::iter::zip;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigInt {
    pub(crate) sign: Sign,
    pub(crate) limbs: Vec<u32>,
}

impl BigInt {
    pub fn new() -> Self {
        Self {
            sign: Sign::Zero,
            limbs: Vec::new(),
        }
    }

    pub fn from_u128(mut n: u128, sign: Sign) -> Self {
        let mask: u128 = u32::MAX as u128;
        let mut chunk: Vec<u32> = vec![0; 4];

        for i in chunk.iter_mut() {
            *i = (n & mask) as u32;
            n >>= 32;
        }
        Self { sign, limbs: chunk }
    }

    fn normalize(&mut self) {
        while let Some(&0) = self.limbs.last() {
            self.limbs.pop();
        }

        if self.limbs.is_empty() {
            self.sign = Sign::Zero;
        }
    }

    fn div_u32(&mut self, divisor: u32) -> u32 {
        let mut acc: u64 = 0;
        for i in self.limbs.iter_mut().rev() {
            acc <<= 32;
            acc += (i.clone()) as u64;
            let rest = acc % (divisor as u64);
            *i = (acc / divisor as u64) as u32;
            acc = rest;
        }
        self.normalize();
        acc as u32
    }

    pub(crate) fn to_owned_string(&self) -> String {
        if self.sign == Sign::Zero {
            return "0".into();
        }
        let mut cloned_value = self.clone();
        let mut actual = String::new();
        while cloned_value.sign != Sign::Zero {
            let rest = cloned_value.div_u32(10);
            actual += format!("{}", rest).as_str();
        }
        if self.sign == Sign::Negative {
            actual.push_str("-");
        }
        let actual = actual.chars().rev().collect::<String>();
        actual
    }

    fn div2_base10(elements: &mut Vec<u8>) -> u8 {
        let mut rest = 0;
        for i in elements.iter_mut().rev() {
            let actual_value = rest * 10 + *i;
            rest = actual_value % 2;
            *i = actual_value / 2;
        }
        while let Some(&0) = elements.last() {
            elements.pop();
        }
        rest as u8
    }

    pub(crate) fn from_slice_str(elements: &str) -> Self {
        let mut vec_elements = elements
            .chars()
            .rev()
            .map(|x| x.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>();
        let mut digits: Vec<u32> = Vec::new();
        let mut binary_rep: Vec<u8> = Vec::new();
        while !vec_elements.is_empty() {
            binary_rep.push(Self::div2_base10(&mut vec_elements) as u8);
        }
        let mut acc: u32 = 0;
        for i in 0..binary_rep.len() {
            acc += (binary_rep[i] as u32) << (i % 32);
            if i % 32 == 31 {
                digits.push(acc);
                acc = 0;
            }
        }
        digits.push(acc);

        Self {
            sign: Sign::Positive,
            limbs: digits,
        }
    }

    pub(crate) fn abs_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.limbs.len() > other.limbs.len() {
            return Some(Ordering::Greater);
        }
        if self.limbs.len() < other.limbs.len() {
            return Some(Ordering::Less);
        }
        for (i, j) in zip(self.limbs.iter().rev(), other.limbs.iter().rev()) {
            if *i > *j {
                return Some(Ordering::Greater);
            }
            if *i < *j {
                return Some(Ordering::Less);
            }
        }
        Some(Ordering::Equal)
    }

    fn abs_sub(&self, other: &Self, sign: Sign) -> Self {
        assert_eq!(
            self.abs_cmp(other),
            Some(Ordering::Greater),
            "the first value must be grater"
        );
        let mut owned: u32 = 0;
        let mut result: Vec<u32> = vec![];
        for i in 0..self.limbs.len() {
            let first = *self.limbs.get(i).unwrap_or(&0) as u32;
            let second = *other.limbs.get(i).unwrap_or(&0) as u32;
            let mut actual: i64 = first as i64 - second as i64 - owned as i64;
            owned = 0;
            if actual < 0 {
                owned += 1;
                actual += 1i64 << 32;
            }
            result.push(actual as u32);
        }
        Self {
            sign,
            limbs: result,
        }
    }

    pub(crate) fn add_same_sign(self, rhs: Self) -> Self {
        assert_eq!(
            self.sign, rhs.sign,
            "For 'add_same_sign' the signs must be equal"
        );
        let mut rest: u32 = 0;
        let mut actual_sum: Vec<u32> = vec![];
        let max_lenght = self.limbs.len().max(rhs.limbs.len());
        for i in 0..max_lenght {
            let first = *self.limbs.get(i).unwrap_or(&0);
            let second = *rhs.limbs.get(i).unwrap_or(&0);
            let actual_value = (first as u64) + (second as u64) + (rest as u64);
            let cell_value = actual_value % ((u32::MAX as u64) + 1);
            actual_sum.push(cell_value as u32);
            rest = (actual_value >> 32) as u32;
        }
        if rest != 0 {
            actual_sum.push(rest);
        }
        Self {
            sign: self.sign,
            limbs: actual_sum,
        }
    }

    pub(crate) fn add_diff_sign(self, rhs: Self) -> Self {
        match self.abs_cmp(&rhs).unwrap_or(Ordering::Equal) {
            Ordering::Equal => {
                return Self {
                    sign: Sign::Zero,
                    limbs: vec![],
                };
            }
            Ordering::Less => {
                let actual_sign = rhs.sign.clone();
                return rhs.abs_sub(&self, actual_sign);
            }
            Ordering::Greater => {
                let actual_sign = self.sign.clone();
                return self.abs_sub(&rhs, actual_sign);
            }
        }
    }
}
