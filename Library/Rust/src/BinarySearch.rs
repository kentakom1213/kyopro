#![allow(dead_code)]

/// # BinarySearch
/// 二分探索の実装
trait BinarySearch<T> {
    fn lower_bound(&self, x: T) -> usize;
    fn upper_bound(&self, x: T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    /// ソート済み配列において、`v`以上の最小のインデックスを取得
    fn lower_bound(&self, v: T) -> usize {
        let mut ng = 1_usize.wrapping_neg();
        let mut ok = self.len();
        while ok.wrapping_sub(ng) > 1 {
            let mid = ng.wrapping_add(ok) / 2;
            if v <= self[mid] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok
    }

    /// ソート済み配列において、`v`より大きい最小のインデックスを取得
    fn upper_bound(&self, v: T) -> usize {
        let mut ng = 1_usize.wrapping_neg();
        let mut ok = self.len();
        while ok.wrapping_sub(ng) > 1 {
            let mid = ng.wrapping_add(ok) / 2;
            if v < self[mid] {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lower_bound() {
        let arr = vec![0, 1, 1, 1, 2, 2, 3, 5];

        assert_eq!(arr.lower_bound(0), 0);
        assert_eq!(arr.lower_bound(1), 1);
        assert_eq!(arr.lower_bound(2), 4);
        assert_eq!(arr.lower_bound(3), 6);
        assert_eq!(arr.lower_bound(4), 7);
        assert_eq!(arr.lower_bound(5), 7);
        assert_eq!(arr.lower_bound(10), 8);
    }

    #[test]
    fn test_upper_bound() {
        let arr = vec![0, 1, 1, 1, 2, 2, 3, 5];

        assert_eq!(arr.upper_bound(0), 1);
        assert_eq!(arr.upper_bound(1), 4);
        assert_eq!(arr.upper_bound(2), 6);
        assert_eq!(arr.upper_bound(3), 7);
        assert_eq!(arr.upper_bound(4), 7);
        assert_eq!(arr.upper_bound(5), 8);
        assert_eq!(arr.upper_bound(10), 8);
    }
}
