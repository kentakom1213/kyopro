#![allow(dead_code)]

use std::collections::{BTreeSet, BTreeMap};
use std::ops::Bound::{Included, Excluded, Unbounded};

/// # MultiSet
/// 多重集合
#[derive(Debug, Clone)]
struct MultiSet<T> {
    counter: BTreeMap<T, usize>,
    multiset: BTreeSet<(T, usize)>,
}

impl<T> MultiSet<T>
where T: Ord + Copy
{
    fn new() -> Self {
        MultiSet {
            counter: BTreeMap::new(),
            multiset: BTreeSet::new(),
        }
    }

    fn insert(&mut self, key: T) {
        // 現在の個数をカウント
        let cnt_key = self.counter.entry(key).or_insert(0);
        // 要素を挿入
        self.multiset.insert((key, *cnt_key));
        // 個数をインクリメント
        *cnt_key += 1;
    }

    fn remove(&mut self, key: T) -> bool {
        match self.counter.get_mut(&key) {
            Some(cnt_key) => {
                if *cnt_key == 0 {
                    false
                } else {
                    *cnt_key -= 1;
                    self.multiset.remove(&(key, *cnt_key));
                    true
                }
            },
            None => false,
        }
    }

    fn is_contain(&self, key: T) -> bool {
        match self.counter.get(&key) {
            Some(&cnt_key) => {
                if cnt_key == 0 {
                    false
                } else {
                    true
                }
            },
            None => false,
        }
    }

    fn is_empty(&self) -> bool {
        self.multiset.is_empty()
    }

    fn len(&self) -> usize {
        self.multiset.len()
    }

    fn first(&self) -> Option<T> {
        match self.multiset.iter().next() {
            Some(&(key, _)) => Some(key),
            None => None,
        }
    }

    fn last(&self) -> Option<T> {
        match self.multiset.iter().next_back() {
            Some(&(key, _)) => Some(key),
            None => None,
        }
    }

    /// x以上の値を探索する
    fn lower_bound(&self, x: T) -> T {
        let mut greater_equal = self.multiset.range(
            (Included((x, 0)), Unbounded)
        );

        match greater_equal.next() {
            Some(&(key, _)) => key,
            None => panic!("No applicable value in this multiset."),
        }
    }

    /// xより大きい値を探索する
    fn upper_bound(&self, x: T) -> T {
        let lb_x = self.lower_bound(x);
        let cnt_key = self.counter[&lb_x];
        let mut greater_equal = self.multiset.range(
            (Excluded((x, cnt_key)), Unbounded)
        );

        match greater_equal.next() {
            Some(&(key, _)) => key,
            None => panic!("No applicable value in this multiset."),
        }
    }

    fn count(&self, key: T) -> usize {
        match self.counter.get(&key) {
            Some(&cnt_key) => cnt_key,
            None => 0,
        }
    }
}

impl<T> IntoIterator for MultiSet<T> {
    type Item = (T, usize);
    type IntoIter = std::collections::btree_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.multiset
            .into_iter()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_usize() {
        let mut mset: MultiSet<usize> = MultiSet::new();

        assert!(mset.is_empty());

        mset.insert(5);
        mset.insert(7);
        mset.insert(7);
        mset.insert(10);
        mset.insert(3);

        // remove value
        assert_eq!(mset.remove(7), true);
        assert_eq!(mset.remove(7), true);
        assert_eq!(mset.remove(0), false);

        // is_contain
        assert_eq!(mset.is_contain(5), true);
        assert_eq!(mset.is_contain(7), false);
        assert_eq!(mset.is_contain(0), false);
        assert_eq!(mset.is_contain(1000), false);

        // first element
        assert_eq!(mset.first(), Some(3));

        assert_eq!(mset.remove(3), true);
        assert_eq!(mset.first(), Some(5));
        assert_eq!(mset.is_contain(3), false);

        // last element
        assert_eq!(mset.last(), Some(10));

        // count values
        mset.insert(20);
        mset.insert(20);
        mset.insert(20);
        /*
         * MultiSet { 3, 5, 10, 20, 20, 20 }
         */

        assert_eq!(mset.count(5), 1);
        assert_eq!(mset.count(20), 3);
        assert_eq!(mset.count(1000), 0);
    }

    #[test]
    fn test_lower_upper() {
        let mut mset = MultiSet::new();

        mset.insert(1);
        mset.insert(2);
        mset.insert(2);
        mset.insert(5);
        mset.insert(5);
        mset.insert(5);
        mset.insert(100);
        mset.insert(1000);
        /*
         * MultiSet { 1, 2, 2, 3, 3, 3, 100, 1000 }
         */

        // around 2
        assert_eq!(mset.lower_bound(2), 2);
        assert_eq!(mset.upper_bound(2), 5);

        // around 3
        assert_eq!(mset.lower_bound(3), 5);
        assert_eq!(mset.upper_bound(3), 5);

        // around 5
        assert_eq!(mset.lower_bound(5), 5);
        assert_eq!(mset.upper_bound(5), 100);

        // around 1000
        assert_eq!(mset.lower_bound(1000), 1000);
    }

    #[test]
    #[should_panic]
    fn test_ub_for_no_val() {
        let mut mset = MultiSet::new();

        mset.insert(100);
        mset.insert(1000);
        /*
         * MultiSet { 100, 1000 }
         */

        mset.upper_bound(1000);
    }

    #[test]
    fn test_into_iter() {
        let mut mset = MultiSet::new();

        mset.insert(1);
        mset.insert(2);
        mset.insert(2);
        mset.insert(5);
        mset.insert(5);
        mset.insert(5);
        mset.insert(100);
        mset.insert(1000);
        /*
         * MultiSet { 1, 2, 2, 3, 3, 3, 100, 1000 }
         */
    
        let mut itr = mset.clone().into_iter();

        assert_eq!(itr.next(), Some((1, 0)));
        assert_eq!(itr.next(), Some((2, 0)));
        assert_eq!(itr.next(), Some((2, 1)));
        assert_eq!(itr.next(), Some((5, 0)));
        assert_eq!(itr.next(), Some((5, 1)));
        assert_eq!(itr.next(), Some((5, 2)));
        assert_eq!(itr.next(), Some((100, 0)));
        assert_eq!(itr.next(), Some((1000, 0)));
        assert_eq!(itr.next(), None);
    }
}
