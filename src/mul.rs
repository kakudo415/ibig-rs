mod div;
mod mul;

use crate::{dword, word};

// carry + pred + lhs * rhs = (ans, carry)
pub(crate) fn pred_carrying_mul(lhs: word, rhs: word, pred: word, carry: word) -> (word, word) {
    let acc: dword = carry as dword + pred as dword + lhs as dword * rhs as dword;
    (acc as word, (acc >> word::BITS) as word)
}

// carry + lhs * rhs = (ans, carry)
pub(crate) fn carrying_mul(lhs: word, rhs: word, carry: word) -> (word, word) {
    let carry = carry as dword;
    let lhs = lhs as dword;
    let rhs = rhs as dword;
    let acc = carry + lhs * rhs;
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
}
