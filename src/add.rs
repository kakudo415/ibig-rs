use std::{cmp, ops};

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

impl ops::Sub for &uHuge {
    type Output = uHuge;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut digits = vec![0; self.digits.len()];
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

// This function will be replaced when std::usize::carrying_add is in stable
pub(crate) fn carrying_add(lhs: word, rhs: word, carry: bool) -> (word, bool) {
    let (acc, c1) = word::overflowing_add(lhs, rhs);
    let (acc, c2) = word::overflowing_add(acc, carry as word);
    (acc, c1 || c2) // Carry will occur at most once
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
    fn carrying_add_0() {
        let lhs = word::MAX;
        let rhs = word::MAX;
        let ans = (word::MAX - 1, true);
        assert_eq!(carrying_add(lhs, rhs, false), ans);
    }

    #[test]
    fn carrying_add_1() {
        let lhs = word::MAX;
        let rhs = word::MAX;
        let ans = (word::MAX, true);
        assert_eq!(carrying_add(lhs, rhs, true), ans);
    }

    #[test]
    fn add_0() {
        let lhs = uHuge {
            digits: vec![12345],
        };
        let rhs = uHuge {
            digits: vec![67890],
        };
        let ans = uHuge {
            digits: vec![80235],
        };
        assert_eq!(&lhs + &rhs, ans);
    }

    #[test]
    fn add_1() {
        let lhs = uHuge {
            digits: vec![word::MAX],
        };
        let rhs = uHuge {
            digits: vec![word::MAX, word::MAX],
        };
        let ans = uHuge {
            digits: vec![word::MAX - 1, 0, 1],
        };
        assert_eq!(&lhs + &rhs, ans);
    }

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
        let lhs = uHuge {
            digits: vec![word::MAX, 0, word::MAX],
        };
        let rhs = uHuge {
            digits: vec![word::MAX, word::MAX],
        };
        let ans = uHuge {
            digits: vec![0, 1, word::MAX - 1],
        };
        assert_eq!(&lhs - &rhs, ans);
    }

    #[test]
    #[should_panic]
    fn sub_1() {
        let lhs = uHuge {
            digits: vec![word::MAX],
        };
        let rhs = uHuge {
            digits: vec![word::MAX, word::MAX],
        };
        let ans = uHuge { digits: vec![0] };
        assert_eq!(&lhs - &rhs, ans);
    }

    #[test]
    fn sub_2() {
        let lhs = uHuge {
            digits: vec![word::MAX, word::MAX, word::MAX],
        };
        let rhs = uHuge {
            digits: vec![0, word::MAX, word::MAX],
        };
        let ans = uHuge {
            digits: vec![word::MAX],
        };
        assert_eq!(&lhs - &rhs, ans);
    }
}
