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
    limbs: Vec<u32>,
}

impl BigInt {
    pub fn new() -> Self {
        Self {
            sign: Sign::Zero,
            limbs: Vec::new(),
        }
    }

    pub fn from_i32(n: i32) -> Self {
        match n {
            0 => Self::new(),
            n if n < 0 => Self::from_u32((-n) as u32, Sign::Negative),
            n => Self::from_u32(n as u32, Sign::Positive),
        }
    }

    pub fn from_u32(n: u32, sign: Sign) -> Self {
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

    fn show(&self) -> String {
        if self.sign == Sign::Zero {
            return "0".into();
        }
        let mut cloned_value = self.clone();
        let mut actual = String::new();
        while cloned_value.sign != Sign::Zero {
            let rest = cloned_value.div_u32(10);
            actual += format!("{}", rest).as_str();
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
    fn from_str(elements: &str) -> Self {
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
}

impl Add for BigInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.sign, Sign::Positive, "For now only positive integers");
        assert_eq!(rhs.sign, Sign::Positive, "For now only positive integers");
        let mut rest: u32 = 0;
        let mut actual_sum: Vec<u32> = vec![];
        let max_lenght = self.limbs.len().max(rhs.limbs.len());
        for i in 0..max_lenght {
            let first = *self.limbs.get(i).unwrap_or(&0);
            let second = *rhs.limbs.get(i).unwrap_or(&0);
            let actual_value = (first as u64) + (second as u64) + (rest as u64);
            let cell_value = actual_value % ((u32::max_value() as u64) + 1);
            actual_sum.push(cell_value as u32);
            rest = (actual_value >> 32) as u32;
        }
        if rest != 0 {
            actual_sum.push(rest);
        }
        Self {
            sign: Sign::Positive, limbs: actual_sum,
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
        let mut big = BigInt::from_i32(i32::max_value());
        let mut value: u64 = i32::max_value() as u64;
        for _ in 0..100 {
            for _ in 0..100 {
                big = big + BigInt::from_i32(i32::max_value());
                value += i32::max_value() as u64;
            }
            assert_eq!(big.show(), format!("{}", value));
        }

        assert_eq!(big.show(), format!("{}", value));
    }
    #[test]
    fn conversion() {
        let number = "19238938192839301829380182038012830810928309180192301298381098209831082038182031802380182093819820380182098093810221892389";
        let big = BigInt::from_str(&number);
        println!("{}", big.show());

        assert_eq!(big.show(), number);
    }
}
