mod add;
mod mul;

use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
pub struct uHuge {
    digits: Vec<usize>,
}

pub(crate) fn pop_zero(digits: &mut Vec<usize>) {
    while let Some(digit) = digits.pop() {
        if digit != 0 {
            digits.push(digit);
            break;
        }
    }
}

impl fmt::Display for uHuge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for digit in self.digits.iter().rev() {
            write!(f, "{:016X} ", digit)?;
        }
        Ok(())
    }
}
