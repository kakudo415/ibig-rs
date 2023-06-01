use std::fmt;

use anyhow::Result;

#[allow(non_camel_case_types)]
pub struct iBig {
    digits: Vec<i8>,
}

impl iBig {
    pub fn from_str(s: &str) -> Result<Self> {
        let mut digits = Vec::new();
        for c in s.chars() {
            digits.push(c.to_digit(10).unwrap() as i8);
        }
        Ok(iBig { digits })
    }
}

impl fmt::Display for iBig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for digit in &self.digits {
            write!(f, "{}", digit)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_to_str() {
        let hoge = iBig::from_str("1234567890").unwrap();
        println!("iBig Display test: {}", hoge);      
    }
}

