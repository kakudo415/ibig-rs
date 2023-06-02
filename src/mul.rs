use std::ops;

use crate::iHuge;

impl ops::Mul for &iHuge {
    type Output = iHuge;

    fn mul(self, rhs: Self) -> Self::Output {
        let len = self.digits.len() + rhs.digits.len();
        let mut digits = Vec::with_capacity(len);
        for (i, ld) in self.digits.iter().enumerate() {
            for (j, rd) in rhs.digits.iter().enumerate() {
                let d = ld * rd;
                if i + j >= digits.len() {
                    digits.push(d);
                    continue;
                }
                digits[i + j] += d;
            }
        }
        iHuge { digits }.carry_and_borrow()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn mul_0() {
        let lhs = iHuge::from_str("2023").unwrap();
        let rhs = iHuge::from_str("2023").unwrap();
        let answer = iHuge::from_str("4092529").unwrap();
        assert_eq!(&lhs * &rhs, answer);
    }

    #[test]
    fn mul_1() {
        let lhs = iHuge::from_str("1024").unwrap();
        let rhs = iHuge::from_str("123456789").unwrap();
        let answer = iHuge::from_str("126419751936").unwrap();
        assert_eq!(&lhs * &rhs, answer);
    }

    #[test]
    fn mul_2() {
        let lhs = iHuge::from_str("18446744073709551615").unwrap();
        let rhs = iHuge::from_str("18446744073709551615").unwrap();
        let answer = iHuge::from_str("340282366920938463426481119284349108225").unwrap();
        assert_eq!(&lhs * &rhs, answer);
    }
}
