use std::{cmp, ops};

use crate::{uHuge, word};

impl ops::Sub for &uHuge {
    type Output = uHuge;

    fn sub(self, rhs: Self) -> Self::Output {
        let len = cmp::max(self.digits.len(), rhs.digits.len());
        let mut digits = vec![0; len];
        sub(&mut digits, &self.digits, &rhs.digits);
        uHuge { digits }.pop_leading_zeros()
    }
}

pub(crate) fn sub(acc: &mut [word], lhs: &[word], rhs: &[word]) {
    let mut borrow = false;
    for i in 0..acc.len() {
        let ld = if lhs.len() > i { lhs[i] } else { 0 };
        let rd = if rhs.len() > i { rhs[i] } else { 0 };
        (acc[i], borrow) = borrowing_sub(ld, rd, borrow);
    }
    if borrow {
        panic!("UNDERFLOW OCCURED");
    }
}

pub(crate) fn sub_assign(acc: &mut [word], rhs: &[word]) {
    let mut borrow = false;
    for i in 0..acc.len() {
        let ad = if acc.len() > i { acc[i] } else { 0 };
        let rd = if rhs.len() > i { rhs[i] } else { 0 };
        (acc[i], borrow) = borrowing_sub(ad, rd, borrow);
    }
}

// This function will be replaced when std::usize::borrowing_sub is in stable
pub(crate) fn borrowing_sub(lhs: word, rhs: word, borrow: bool) -> (word, bool) {
    let (acc, c1) = word::overflowing_sub(lhs, rhs);
    let (acc, c2) = word::overflowing_sub(acc, borrow as word);
    (acc, c1 || c2) // Borrow will occur at most once
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrowing_sub_0() {
        let lhs = 0;
        let rhs = word::MAX;
        let ans = (1, true);
        assert_eq!(borrowing_sub(lhs, rhs, false), ans);
    }

    #[test]
    fn borrowing_sub_1() {
        let lhs = 0;
        let rhs = word::MAX;
        let ans = (0, true);
        assert_eq!(borrowing_sub(lhs, rhs, true), ans);
    }
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
