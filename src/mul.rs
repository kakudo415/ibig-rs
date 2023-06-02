use std::ops;

use crate::{dword, pop_zero, uHuge, word};

impl ops::Mul for &uHuge {
    type Output = uHuge;

    fn mul(self, rhs: Self) -> Self::Output {
        let len = self.digits.len() + rhs.digits.len();
        let mut digits = vec![0; len];
        mul(&mut digits, &self.digits, &rhs.digits);
        uHuge { digits }
    }
}

fn mul(acc: &mut Vec<word>, lhs: &Vec<word>, rhs: &Vec<word>) {
    for (i, ld) in lhs.iter().enumerate() {
        let mut carry = 0;
        for (j, rd) in rhs.iter().enumerate() {
            (acc[i + j], carry) = pred_carrying_mul(*ld, *rd, acc[i + j], carry);
        }
        acc[i + rhs.len()] = carry;
    }
    pop_zero(acc);
}

// carry + pred + lhs * rhs = (ans, carry)
pub(crate) fn pred_carrying_mul(lhs: word, rhs: word, pred: word, carry: word) -> (word, word) {
    let acc: dword = carry as dword + pred as dword + lhs as dword * rhs as dword;
    (acc as word, (acc >> word::BITS) as word)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pred_carrying_mul_0() {
        let lhs = word::MAX;
        let rhs = word::MAX;
        let pred = word::MAX;
        let carry = word::MAX;
        let ans = (word::MAX, word::MAX);
        assert_eq!(pred_carrying_mul(lhs, rhs, pred, carry), ans);
    }

    #[test]
    fn mul_0() {
        let lhs = uHuge {
            digits: vec![12345],
        };
        let rhs = uHuge {
            digits: vec![67890],
        };
        let ans = uHuge {
            digits: vec![838102050],
        };
        assert_eq!(&lhs * &rhs, ans);
    }

    #[test]
    fn mul_1() {
        let lhs = uHuge {
            digits: vec![word::MAX],
        };
        let rhs = uHuge {
            digits: vec![word::MAX],
        };
        let ans = uHuge {
            digits: vec![1, 18446744073709551614],
        };
        assert_eq!(&lhs * &rhs, ans);
    }

    #[test]
    fn mul_2() {
        let lhs = uHuge {
            digits: vec![word::MAX],
        };
        let rhs = uHuge {
            digits: vec![word::MAX; 4],
        };
        let ans = uHuge {
            digits: vec![1, word::MAX, word::MAX, word::MAX, word::MAX - 1],
        };
        assert_eq!(&lhs * &rhs, ans);
    }

    #[test]
    fn mul_3() {
        let lhs = uHuge {
            digits: vec![word::MAX; 3],
        };
        let rhs = uHuge {
            digits: vec![word::MAX; 5],
        };
        let ans = uHuge {
            digits: vec![
                1,
                0,
                0,
                word::MAX,
                word::MAX,
                word::MAX - 1,
                word::MAX,
                word::MAX,
            ],
        };
        assert_eq!(&lhs * &rhs, ans);
    }

    #[test]
    fn mul_4() {
        let lhs = uHuge { digits: vec![1; 3] };
        let rhs = uHuge { digits: vec![1; 5] };
        let ans = uHuge {
            digits: vec![1, 2, 3, 3, 3, 2, 1],
        };
        assert_eq!(&lhs * &rhs, ans);
    }
}
