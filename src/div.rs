use std::ops;

use crate::mul::{mul_assign_n1, mul_nn};
use crate::sub::sub_assign;
use crate::{pop_zero, uHuge, word};

impl ops::Div for &uHuge {
    type Output = uHuge;

    fn div(self, rhs: Self) -> Self::Output {
        let mut digits = div(&self.digits, &rhs.digits);
        pop_zero(&mut digits);
        uHuge { digits }
    }
}

pub(crate) fn div(lhs: &Vec<word>, rhs: &Vec<word>) -> Vec<word> {
    let inverse = fixed_inverse(&rhs);
    let mut acc = vec![0; lhs.len() + inverse.len()];
    mul_nn(&mut acc, lhs, &inverse);
    acc[inverse.len()..].to_vec() // remove digits after the point
}

// Calculate inverse of op in fixed point number by newton-raphson method
pub(crate) fn fixed_inverse(op: &[word]) -> Vec<word> {
    let mut inverse = vec![0; op.len() * 2];
    let mut pred = vec![0; op.len() * 2];
    let mut acc0 = vec![0; op.len() * 4];
    let mut acc1 = vec![0; op.len() * 6];

    pred[op.len()] = 1; // initial predict
    'check_accuracy: loop {
        mul_nn(&mut acc0[..op.len() * 4], &pred, &pred);
        mul_assign_n1(&mut pred, 2);
        mul_nn(&mut acc1[..op.len() * 6], &acc0[..op.len() * 4], &op);
        sub_assign(&mut pred, &acc1[op.len() * 2..op.len() * 4]);

        for i in 0..inverse.len() {
            if inverse[i] != pred[i] {
                inverse.copy_from_slice(&pred);
                continue 'check_accuracy;
            }
        }
        break;
    }
    inverse
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_inverse_0() {
        // 0x02 -> 0x00.80 00
        let operand = uHuge::from_str("2").unwrap();
        let ans = uHuge {
            digits: vec![0, 1 << (word::BITS - 1)],
        };
        assert_eq!(fixed_inverse(&operand.digits), ans.digits);
    }

    #[test]
    fn div_0() {
        let lhs = uHuge::from_str("123").unwrap();
        let rhs = uHuge::from_str("13").unwrap();
        let ans = uHuge::from_str("F").unwrap();
        assert_eq!(&lhs / &rhs, ans);
    }

    #[test]
    fn div_1() {
        let lhs = uHuge::from_str("1234567890").unwrap();
        let rhs = uHuge::from_str("ABCDE").unwrap();
        let ans = uHuge::from_str("1B203").unwrap();
        assert_eq!(&lhs / &rhs, ans);
    }
}
