mod add;
pub mod sub;

use crate::word;

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
}
