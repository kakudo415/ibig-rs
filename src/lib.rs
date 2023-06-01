mod add;

use std::fmt;

use anyhow::Result;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
pub struct iHuge {
    digits: Vec<i8>,
}

impl iHuge {
    pub fn from_str(s: &str) -> Result<Self> {
        let mut digits = Vec::new();
        for c in s.chars().rev() {
            digits.push(c.to_digit(10).unwrap() as i8);
        }
        Ok(iHuge { digits })
    }
}

impl fmt::Display for iHuge {
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
        let hoge = iHuge::from_str("1234567890").unwrap();
        println!("iHuge Display test: {}", hoge);
    }
}
