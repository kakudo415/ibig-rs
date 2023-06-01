mod add;

use std::{cmp, fmt, ops};

use anyhow::Result;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
pub struct iBig {
    digits: Vec<i8>,
}

impl iBig {
    pub fn from_str(s: &str) -> Result<Self> {
        let mut digits = Vec::new();
        for c in s.chars().rev() {
            digits.push(c.to_digit(10).unwrap() as i8);
        }
        Ok(iBig { digits })
    }
}

impl fmt::Display for iBig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for digit in self.digits.iter().rev() {
            write!(f, "{}", digit)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_0() {
        let hoge = iBig::from_str("1234567890").unwrap();
        println!("iBig Display test: {}", hoge);
    }
}
