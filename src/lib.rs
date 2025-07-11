use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Mul, Sub};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sign {
    Positive,
    Negative,
    Zero,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigInt {
    sign: Sign,
    limbs: Vec<u64>,
}

impl BigInt {
    pub fn new() -> Self {
        Self {
            sign: Sign::Zero,
            limbs: Vec::new(),
        }
    }

    pub fn from_i64(n: i64) -> Self {
        match n {
            0 => Self::new(),
            n if n < 0 => Self::from_u64((-n) as u64, Sign::Negative),
            n => Self::from_u64(n as u64, Sign::Positive),
        }
    }

    pub fn from_u64(n: u64, sign: Sign) -> Self {
        if sign == Sign::Zero {
            return Self::new();
        }
        Self {
            sign,
            limbs: vec![n],
        }
    }

    fn normalize(&mut self) {
        // Remove leading zeros
        while let Some(&0) = self.limbs.last() {
            self.limbs.pop();
        }

        if self.limbs.is_empty() {
            self.sign = Sign::Zero;
        }
    }

    fn div_u64(&mut self, divisor: u64) -> u64 {
        let mut acc: u128 = 0;
        for i in self.limbs.iter_mut().rev() {
            acc <<= 64;
            acc += (i.clone()) as u128;
            let rest = acc % (divisor as u128);
            *i = (acc / divisor as u128) as u64;
            acc = rest;
        }
        self.normalize();
        acc as u64
    }

    fn show(&self) -> String {
        if self.sign == Sign::Zero {
            return "0".into();
        }
        let mut cloned_value = self.clone();
        let mut actual = String::new();
        while cloned_value.sign != Sign::Zero {
            let rest = cloned_value.div_u64(10);
            actual += format!("{}", rest).as_str();
        }
        let actual = actual.chars().rev().collect::<String>();
        actual
    }
}

impl Add for BigInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.sign, Sign::Positive, "For now only positive integers");
        assert_eq!(rhs.sign, Sign::Positive, "For now only positive integers");
        let mut rest: u64 = 0;
        let mut actual_sum: Vec<u64> = vec![];
        let max_lenght = self.limbs.len().max(rhs.limbs.len());
        for i in 0..max_lenght {
            let first = *self.limbs.get(i).unwrap_or(&0);
            let second = *rhs.limbs.get(i).unwrap_or(&0);
            let actual_value = (first as u128) + (second as u128) + (rest as u128);
            let cell_value = actual_value % ((u64::max_value() as u128) + 1);
            actual_sum.push(cell_value as u64);
            rest = (actual_value >> 64) as u64;
        }
        if rest != 0 {
            actual_sum.push(rest);
        }
        Self {
            sign: Sign::Positive,
            limbs: actual_sum,
        }
    }
}

impl Sub for BigInt {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Mul for BigInt {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut big = BigInt::from_i64(i64::max_value());
        let mut value: u128 = i64::max_value() as u128;
        for _ in 0..100 {
            big = big + BigInt::from_i64(i64::max_value());
            value += i64::max_value() as u128;
            assert_eq!(big.show(), format!("{}", value));
        }

        assert_eq!(0, 0);
    }
}
