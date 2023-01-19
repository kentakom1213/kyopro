#![allow(dead_code)]

use std::collections::BTreeSet;
use std::ops::Bound::{Included, Excluded, Unbounded};

/// # BinarySearch
/// 二分探索
trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> Option<&T>;
    fn upper_bound(&self, x: &T) -> Option<&T>;
}

impl<T: Ord> BinarySearch<T> for BTreeSet<T> {
    /// x以上の値を探索する
    fn lower_bound(&self, x: &T) -> Option<&T> {
        let mut greater_equal = self.range(
            (Included(x), Unbounded)
        );

        greater_equal.next()
    }

    /// xより大きい値を探索する
    fn upper_bound(&self, x: &T) -> Option<&T> {
        let mut greater_equal = self.range(
            (Excluded(x), Unbounded)
        );

        greater_equal.next()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lower_bound() {
        let mut set = BTreeSet::new();
        set.insert(5);
        set.insert(9);
        set.insert(200);
        set.insert(200);
        set.insert(-5);
        /*
         * BTreeSet{ -5, 5, 9, 200, 200, }
         */

        assert_eq!(set.lower_bound(&4), Some(&5));
        assert_eq!(set.lower_bound(&5), Some(&5));
        assert_eq!(set.lower_bound(&8), Some(&9));
        assert_eq!(set.lower_bound(&100), Some(&200));
        assert_eq!(set.lower_bound(&200), Some(&200));
        assert_eq!(set.lower_bound(&201), None);
    }

    #[test]
    fn test_upper_bound() {
        let mut set = BTreeSet::new();
        set.insert(5);
        set.insert(9);
        set.insert(200);
        set.insert(200);
        set.insert(-5);
        /*
         * BTreeSet{ -5, 5, 9, 200, 200, }
         */

        assert_eq!(set.upper_bound(&4), Some(&5));
        assert_eq!(set.upper_bound(&5), Some(&9));
        assert_eq!(set.upper_bound(&8), Some(&9));
        assert_eq!(set.lower_bound(&100), Some(&200));
        assert_eq!(set.upper_bound(&200), None);
        assert_eq!(set.upper_bound(&201), None);
    }
}
