use std::{cmp, ops};

use super::carrying_add;
use crate::{pop_zero, uHuge, word};

impl ops::Add for &uHuge {
    type Output = uHuge;

    fn add(self, rhs: Self) -> Self::Output {
        let len = cmp::max(self.digits.len(), rhs.digits.len()) + 1;
        let mut digits = vec![0; len];
        add(&mut digits, &self.digits, &rhs.digits);
        uHuge { digits }
    }
}

fn add(acc: &mut Vec<word>, lhs: &Vec<word>, rhs: &Vec<word>) {
    let mut carry = false;
    for i in 0..acc.len() {
        let ld = if lhs.len() > i { lhs[i] } else { 0 };
        let rd = if rhs.len() > i { rhs[i] } else { 0 };
        (acc[i], carry) = carrying_add(ld, rd, carry);
    }
    pop_zero(acc);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn add_0() {
        let lhs = uHuge::from_str("12345").unwrap();
        let rhs = uHuge::from_str("67890").unwrap();
        let ans = uHuge::from_str("79BD5").unwrap();
        assert_eq!(&lhs + &rhs, ans);
    }

    #[test]
    fn add_1() {
        let lhs = uHuge::from_str("FFFFFFFFFFFFFFFF").unwrap();
        let rhs = uHuge::from_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap();
        let ans = uHuge::from_str("00000000000000010000000000000000FFFFFFFFFFFFFFFE").unwrap();
        assert_eq!(&lhs + &rhs, ans);
    }
}
