use std::{cmp, ops};

use crate::iHuge;

impl ops::Add for &iHuge {
    type Output = iHuge;

    fn add(self, rhs: Self) -> Self::Output {
        let len = cmp::max(self.digits.len(), rhs.digits.len());
        let mut digits = Vec::with_capacity(len + 1);
        for i in 0..len {
            let ld = if self.digits.len() > i {
                self.digits[i]
            } else {
                0
            };
            let rd = if rhs.digits.len() > i {
                rhs.digits[i]
            } else {
                0
            };
            digits.push(ld + rd);
        }
        iHuge { digits }.carry_and_borrow()
    }
}

impl ops::Sub for &iHuge {
    type Output = iHuge;

    fn sub(self, rhs: Self) -> Self::Output {
        let len = cmp::max(self.digits.len(), rhs.digits.len());
        let mut digits = Vec::with_capacity(len + 1);
        for i in 0..len {
            let ld = if self.digits.len() > i {
                self.digits[i]
            } else {
                0
            };
            let rd = if rhs.digits.len() > i {
                rhs.digits[i]
            } else {
                0
            };
            digits.push(ld - rd);
        }
        iHuge { digits }.carry_and_borrow()
    }
}

impl iHuge {
    pub(crate) fn carry_and_borrow(mut self) -> Self {
        for i in 0..(self.digits.len() - 1) {
            if self.digits[i] >= 10 {
                self.digits[i + 1] += self.digits[i] / 10;
                self.digits[i] %= 10;
            }
            if self.digits[i] <= -1 {
                let b = (-self.digits[i] - 1) / 10 + 1;
                self.digits[i + 1] -= b;
                self.digits[i] += 10 * b;
            }
        }
        while let Some(highest) = self.digits.pop() {
            if highest < 10 {
                self.digits.push(highest);
                break;
            }
            self.digits.push(highest % 10);
            self.digits.push(highest / 10);
        }
        while let Some(highest) = self.digits.pop() {
            if highest != 0 {
                self.digits.push(highest);
                break;
            }
        }
        if self.digits.len() == 0 {
            self.digits.push(0);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn add_0() {
        let lhs = iHuge::from_str("2023").unwrap();
        let rhs = iHuge::from_str("601").unwrap();
        let answer = iHuge::from_str("2624").unwrap();
        assert_eq!(&lhs + &rhs, answer);
    }

    #[test]
    fn add_with_carry_0() {
        let lhs = iHuge::from_str("999").unwrap();
        let rhs = iHuge::from_str("415").unwrap();
        let answer = iHuge::from_str("1414").unwrap();
        assert_eq!(&lhs + &rhs, answer);
    }

    #[test]
    fn add_with_carry_1() {
        let lhs = iHuge::from_str("999").unwrap();
        let rhs = iHuge::from_str("999").unwrap();
        let answer = iHuge::from_str("1998").unwrap();
        assert_eq!(&lhs + &rhs, answer);
    }

    #[test]
    fn sub_0() {
        let lhs = iHuge::from_str("999").unwrap();
        let rhs = iHuge::from_str("415").unwrap();
        let answer = iHuge::from_str("584").unwrap();
        assert_eq!(&lhs - &rhs, answer);
    }

    #[test]
    fn sub_with_borrow_0() {
        let lhs = iHuge::from_str("584").unwrap();
        let rhs = iHuge::from_str("495").unwrap();
        let answer = iHuge::from_str("89").unwrap();
        assert_eq!(&lhs - &rhs, answer);
    }
}
