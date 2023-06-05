use std::ops;

use crate::mul::{mul_assign_n1, mul_nn};
use crate::sub::sub_assign;
use crate::{uHuge, word};

impl ops::Div for &uHuge {
    type Output = uHuge;

    fn div(self, rhs: Self) -> Self::Output {
        let digits = div(&self.digits, &rhs.digits);
        uHuge { digits }.pop_leading_zeros()
    }
}

pub(crate) fn div(lhs: &[word], rhs: &[word]) -> Vec<word> {
    let inverse = fixed_inverse(rhs, lhs.len() + 1);
    let mut acc = vec![0; lhs.len() + inverse.len()];
    mul_nn(&mut acc, lhs, &inverse);
    acc[inverse.len()..].to_vec() // remove digits after the point
}

// Calculate inverse of op in fixed point number by Newton-Raphson method
pub(crate) fn fixed_inverse(op: &[word], precision: usize) -> Vec<word> {
    let len = op.len() + precision;
    let mut inverse = vec![0; len];
    let mut pred = vec![0; len];
    let mut acc0 = vec![0; len * 2];
    let mut acc1 = vec![0; len * 3];

    pred[op.len()] = 1; // initial predict
    'check_accuracy: loop {
        mul_nn(&mut acc0[..len * 2], &pred, &pred);
        mul_assign_n1(&mut pred, 2);
        mul_nn(&mut acc1[..len * 3], &acc0[..len * 2], op);
        sub_assign(&mut pred, &acc1[len..len * 2]);

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
        assert_eq!(fixed_inverse(&operand.digits, 1), ans.digits);
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

    #[test]
    fn div_random_0() {
        let lhs = uHuge::from_str("123285196341321511").unwrap();
        let rhs = uHuge::from_str("2561").unwrap();
        let ans = uHuge::from_str("7CA130980b6012").unwrap();
        assert_eq!(&lhs / &rhs, ans);
    }

    #[test]
    fn div_random_1() {
        let lhs = uHuge::from_str("A7868AB7678696D697F796EC99697654321BAEF").unwrap();
        let rhs = uHuge::from_str("BB5A6C4E4F798E796C9").unwrap();
        let ans = uHuge::from_str("e4e84cf1646d00e7e901").unwrap();
        assert_eq!(&lhs / &rhs, ans);
    }

    #[test]
    fn div_random_2() {
        let lhs = uHuge::from_str("835a7ee8988b46291543d807c247d109d558df0e878c37d6bc08dde14862a59f13c7048c8e68a9db259132eb5e3aee6ffbe123eecbebbb2c39d5f57c6f28e43faa8434f4cb2efecbd47a803be09fb5d158ec7a2919607fe653ecc28f34ea525b6b7bead2").unwrap();
        let rhs = uHuge::from_str("9c68655e6d90976abb0167cead8613939fc37b918ee66f000fbb3bfdcc3974e9d66926b79f8d59b3475270154db391af92b4").unwrap();
        let ans = uHuge::from_str("d6fe11555b72033a7614437ca5c8e3d6281ce8aca5375a721cadf5d734cbd0b4c97d7af42881a24a8da200cda47b9355b672").unwrap();
        assert_eq!(&lhs / &rhs, ans);
    }

    #[test]
    fn div_random_3() {
        let lhs = uHuge::from_str("c1395fded3d2baaf75a984226478e0177b293931a6942b3362d55988f54ab9ab801d9384a3a46bad989dbad1d24bc89b580efaee326468351bb6e4a00d0001d91453b784812a0c0ee1f215e79624227f8be133dc0742d9081ae28e381a7e92154e979919d847dd37ec075b66fb65c254e800777308c4b52e7ac94622d88f816088ee1e9bd436413e4cb2efd8d84560c52e8852c4624de44f7fc960181a5785825287ee8861eae5f3da72f692afed3eda0284dc0e672a27338be6ce916be545e020fc3455871c09ac363e8d3e6214b3ad78d9dfeccf8864578dd2748c53c57805ac398b727e31f324b9109a2d16077f1a900cb86f4045418b099e1e070511e0d749133f989cafc2e0807bb87ebbd585bafc62ffbc2849e926f7130227fb6fe426e1edc0f4dcbe9573ea50fc2d").unwrap();
        let rhs = uHuge::from_str("3f2fc41922c3fad436801d8019497b9dd1e0145aa800be071c972f82accbce0a1f3eb344759f2375031a969b20bfbf887ce6f23327f017d179ac74da7f05dd730f2a169f68fc52817dacc5fa8a09c77497eb860c0f922b1dc9ddb1a7baea1f9c3fde2a00").unwrap();
        let ans = uHuge::from_str("30ed89c2e83030215c49af1f820e1bad559bf4fffbad47707f583dddb2c0071874985319df248a5a12f3aa1c83da404ab88055c550f3820e1b381a9d00b2561fab6b25e43e1d46e5aa57fa4c4624eee6afd1bcc42f8e7603bd899beff0096ae9891ba9734be59864869828f70ba606b3cd295f80cd2201efc067de00f4cdfd882fb3a6696f2fe62804e04699cd5243f4b91f6f3d4060de1a4e071313886e48ae0a1e803a7a67783f5bd868ff409df0b7cc390597d59ae7f288c722143389687b09a0edb4132382c08").unwrap();
        assert_eq!(&lhs / &rhs, ans);
    }
}
