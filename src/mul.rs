use std::ops;

use crate::{pop_zero, uHuge};

impl ops::Mul for &uHuge {
    type Output = uHuge;

    fn mul(self, rhs: Self) -> Self::Output {
        let len = self.digits.len() + rhs.digits.len();
        let mut digits = vec![0; len];
        mul(&mut digits, &self.digits, &rhs.digits);
        uHuge { digits }
    }
}

fn mul(acc: &mut Vec<usize>, lhs: &Vec<usize>, rhs: &Vec<usize>) {
    for (i, ld) in lhs.iter().enumerate() {
        let mut carry = 0;
        for (j, rd) in rhs.iter().enumerate() {
            (acc[i + j], carry) = mulpc(*ld, *rd, acc[i + j], carry);
        }
        acc[i + rhs.len()] = carry;
    }
    pop_zero(acc);
}

// carry + pred + lhs * rhs = (ans, carry)
pub(crate) fn mulpc(lhs: usize, rhs: usize, pred: usize, carry: usize) -> (usize, usize) {
    // FIXME: u128 is not necessarily twice the bit size of usize
    let acc: u128 = carry as u128 + pred as u128 + lhs as u128 * rhs as u128;
    (acc as usize, (acc >> 64) as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mulpc_0() {
        let lhs = usize::MAX;
        let rhs = usize::MAX;
        let pred = usize::MAX;
        let carry = usize::MAX;
        let ans = (usize::MAX, usize::MAX);
        assert_eq!(mulpc(lhs, rhs, pred, carry), ans);
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
            digits: vec![usize::MAX],
        };
        let rhs = uHuge {
            digits: vec![usize::MAX],
        };
        let ans = uHuge {
            digits: vec![1, 18446744073709551614],
        };
        assert_eq!(&lhs * &rhs, ans);
    }

    #[test]
    fn mul_2() {
        let lhs = uHuge {
            digits: vec![usize::MAX],
        };
        let rhs = uHuge {
            digits: vec![usize::MAX; 4],
        };
        let ans = uHuge {
            digits: vec![1, usize::MAX, usize::MAX, usize::MAX, usize::MAX - 1],
        };
        assert_eq!(&lhs * &rhs, ans);
    }

    #[test]
    fn mul_3() {
        let lhs = uHuge {
            digits: vec![usize::MAX; 3],
        };
        let rhs = uHuge {
            digits: vec![usize::MAX; 5],
        };
        let ans = uHuge {
            digits: vec![
                1,
                0,
                0,
                usize::MAX,
                usize::MAX,
                usize::MAX - 1,
                usize::MAX,
                usize::MAX,
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
