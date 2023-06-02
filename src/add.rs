use std::{cmp, ops};

use crate::{pop_zero, uHuge};

impl ops::Add for &uHuge {
    type Output = uHuge;

    fn add(self, rhs: Self) -> Self::Output {
        let len = cmp::max(self.digits.len(), rhs.digits.len()) + 1;
        let mut digits = vec![0; len];
        add(&mut digits, &self.digits, &rhs.digits);
        uHuge { digits }
    }
}

fn add(acc: &mut Vec<usize>, lhs: &Vec<usize>, rhs: &Vec<usize>) {
    let mut carry = false;
    for i in 0..acc.len() {
        let ld = if lhs.len() > i { lhs[i] } else { 0 };
        let rd = if rhs.len() > i { rhs[i] } else { 0 };
        (acc[i], carry) = addc(ld, rd, carry);
    }
    pop_zero(acc);
}

pub(crate) fn addc(lhs: usize, rhs: usize, carry: bool) -> (usize, bool) {
    let (acc, c1) = usize::overflowing_add(lhs, rhs);
    let (acc, c2) = usize::overflowing_add(acc, carry as usize);
    (acc, c1 || c2) // Carry will occur at most once
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addc_0() {
        let lhs = usize::MAX;
        let rhs = usize::MAX;
        let ans = (usize::MAX - 1, true);
        assert_eq!(addc(lhs, rhs, false), ans);
    }

    #[test]
    fn addc_1() {
        let lhs = usize::MAX;
        let rhs = usize::MAX;
        let ans = (usize::MAX, true);
        assert_eq!(addc(lhs, rhs, true), ans);
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
            digits: vec![usize::MAX],
        };
        let rhs = uHuge {
            digits: vec![usize::MAX, usize::MAX],
        };
        let ans = uHuge {
            digits: vec![usize::MAX - 1, 0, 1],
        };
        assert_eq!(&lhs + &rhs, ans);
    }
}
