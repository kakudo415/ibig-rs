mod add;
mod mul;

use std::fmt;

#[cfg(target_pointer_width = "64")]
#[allow(non_camel_case_types)]
type word = u64;

#[cfg(target_pointer_width = "64")]
#[allow(non_camel_case_types)]
type dword = u128;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
pub struct uHuge {
    digits: Vec<word>,
}

pub(crate) fn pop_zero(digits: &mut Vec<word>) {
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
            write!(f, "{:0w$X} ", digit, w = word::BITS as usize)?;
        }
        Ok(())
    }
}
