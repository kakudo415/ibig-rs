mod add;
mod mul;

use std::fmt;

use anyhow::{Context, Result};

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
pub struct iHuge {
    digits: Vec<isize>,
}

impl iHuge {
    pub fn from_str(s: &str) -> Result<Self> {
        let mut digits = Vec::new();
        for c in s.chars().rev() {
            digits.push(c.to_digit(10).context("Parse Error")? as isize);
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
