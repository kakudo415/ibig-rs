use std::{cmp, ops};

use super::borrowing_sub;
use crate::{pop_zero, uHuge, word};

impl ops::Sub for &uHuge {
    type Output = uHuge;

    fn sub(self, rhs: Self) -> Self::Output {
        let len = cmp::max(self.digits.len(), rhs.digits.len());
        let mut digits = vec![0; len];
        sub(&mut digits, &self.digits, &rhs.digits);
        uHuge { digits }
    }
}

fn sub(acc: &mut Vec<word>, lhs: &Vec<word>, rhs: &Vec<word>) {
    let mut borrow = false;
    for i in 0..acc.len() {
        let ld = if lhs.len() > i { lhs[i] } else { 0 };
        let rd = if rhs.len() > i { rhs[i] } else { 0 };
        (acc[i], borrow) = borrowing_sub(ld, rd, borrow);
    }
    if borrow {
        panic!("UNDERFLOW OCCURED");
    }
    pop_zero(acc);
}

pub(crate) fn sub_assign(acc: &mut [word], rhs: &[word]) {
    let mut borrow = false;
    for i in 0..acc.len() {
        let ad = if acc.len() > i { acc[i] } else { 0 };
        let rd = if rhs.len() > i { rhs[i] } else { 0 };
        (acc[i], borrow) = borrowing_sub(ad, rd, borrow);
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn sub_0() {
        let lhs = uHuge::from_str("FFFFFFFFFFFFFFFF0000000000000000FFFFFFFFFFFFFFFF").unwrap();
        let rhs = uHuge::from_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap();
        let ans = uHuge::from_str("FFFFFFFFFFFFFFFE00000000000000010000000000000000").unwrap();
        assert_eq!(&lhs - &rhs, ans);
    }

    #[test]
    #[should_panic]
    fn sub_1() {
        let lhs = uHuge::from_str("FFFFFFFFFFFFFFFF").unwrap();
        let rhs = uHuge::from_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap();
        let ans = uHuge::from_str("0").unwrap();
        assert_eq!(&lhs - &rhs, ans);
    }

    #[test]
    fn sub_2() {
        let lhs = uHuge::from_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap();
        let rhs = uHuge::from_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF0000000000000000").unwrap();
        let ans = uHuge::from_str("FFFFFFFFFFFFFFFF").unwrap();
        assert_eq!(&lhs - &rhs, ans);
    }
}
