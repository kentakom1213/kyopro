#![allow(non_snake_case)]

use proconio::{fastout, input};

use crate::mex_set::MexSet;

#[fastout]
fn main() {
    input! {
        N: usize,
        P: [isize; N]
    }

    let mut mset = MexSet::new();

    for &p in &P {
        // piを追加
        mset.insert(p);
        // mexを求める
        let res = mset.mex(0);

        println!("{res}");
    }
}

mod mex_set {
    #![allow(dead_code)]
    //! Mexを管理するデータ構造
    use std::{
        collections::BTreeSet,
        ops::{Bound::*, RangeBounds},
    };
    /// 集合とそのmexを管理する
    #[derive(Debug)]
    pub struct MexSet {
        pub ranges: BTreeSet<(isize, isize)>,
    }
    impl MexSet {
        const INF: isize = isize::MIN;
        const SUP: isize = isize::MAX;
        /// MexSetを初期化する
        pub fn new() -> Self {
            let ranges = [(Self::INF, Self::INF), (Self::SUP, Self::SUP)]
                .into_iter()
                .collect();
            Self { ranges }
        }
        /// 集合に要素`x`を追加する
        /// ### 戻り値
        /// - `true`: `x`が追加された場合
        /// - `false`: `x`がすでに存在していた場合
        pub fn insert(&mut self, x: isize) -> bool {
            // 範囲が`(INF, SUP)` である場合 → 追加されない
            if self.ranges.len() == 1 {
                return false;
            }
            let &(ll, l) = self.ranges.range(..(x + 1, x + 1)).next_back().unwrap();
            let &(r, rr) = self.ranges.range((x + 1, x + 1)..).next().unwrap();
            if x <= l {
                return false;
            }
            match (l == x - 1, x + 1 == r) {
                (false, false) => {
                    self.ranges.insert((x, x));
                }
                (false, true) => {
                    self.ranges.remove(&(r, rr));
                    self.ranges.insert((x, rr));
                }
                (true, false) => {
                    self.ranges.remove(&(ll, l));
                    self.ranges.insert((ll, x));
                }
                (true, true) => {
                    self.ranges.remove(&(ll, l));
                    self.ranges.remove(&(r, rr));
                    self.ranges.insert((ll, rr));
                }
            }
            true
        }
        /// 集合から要素`x`を削除する
        /// ### 戻り値
        /// - `true`: `x`が削除された場合
        /// - `false`: `x`がすでに存在していなかった場合
        pub fn delete(&mut self, x: isize) -> bool {
            let &(ll, l) = self.ranges.range(..(x + 1, x + 1)).next_back().unwrap();
            if l < x {
                return false;
            }
            self.ranges.remove(&(ll, l));
            match (ll == x, x == l) {
                (false, false) => {
                    self.ranges.insert((ll, x - 1));
                    self.ranges.insert((x + 1, l));
                }
                (false, true) => {
                    self.ranges.insert((ll, x - 1));
                }
                (true, false) => {
                    self.ranges.insert((x + 1, l));
                }
                (true, true) => (),
            }
            true
        }
        #[inline]
        fn parse_range<R: RangeBounds<isize>>(range: R) -> (isize, isize) {
            let start = match range.start_bound() {
                Unbounded => Self::INF,
                Included(&s) => s,
                Excluded(&s) => s + 1,
            };
            let end = match range.end_bound() {
                Unbounded => Self::SUP,
                Included(&e) => e,
                Excluded(&e) => e - 1,
            };
            (start, end)
        }
        /// 集合に区間を追加する
        /// - 計算量: O(log(n)) (amotized)
        pub fn insert_range<R: RangeBounds<isize>>(&mut self, range: R) -> bool {
            let (start, end) = Self::parse_range(range);
            if start > end {
                return false;
            }
            // 範囲が`(INF, SUP)` である場合 → 追加されない
            if self.ranges.len() == 1 {
                return false;
            }
            // 全範囲を追加する場合
            if (start, end) == (Self::INF, Self::SUP) {
                let len = self.ranges.len();
                self.ranges.clear();
                self.ranges.insert((Self::INF, Self::SUP));
                return len != self.ranges.len();
            }
            if start == end {
                return self.insert(start);
            }
            while let Some(&(l, r)) = self
                .ranges
                .range((Excluded((start, start)), Excluded((end, end))))
                .next()
            {
                if r < end {
                    self.ranges.remove(&(l, r));
                } else {
                    break;
                }
            }
            let &(ll, l) = self
                .ranges
                .range(..(start + 1, start + 1))
                .next_back()
                .unwrap();
            let &(r, rr) = self.ranges.range((end, end)..).next().unwrap();
            if ll <= start && end <= l {
                return false;
            }
            match (start <= l + 1, r - 1 <= end) {
                (false, false) => {
                    self.ranges.insert((start, end));
                }
                (false, true) => {
                    self.ranges.remove(&(r, rr));
                    self.ranges.insert((start, rr));
                }
                (true, false) => {
                    self.ranges.remove(&(ll, l));
                    self.ranges.insert((ll, end));
                }
                (true, true) => {
                    self.ranges.remove(&(ll, l));
                    self.ranges.remove(&(r, rr));
                    self.ranges.insert((ll, rr));
                }
            }
            true
        }
        /// 集合から区間を削除する
        /// - 計算量: O(log(n)) (amotized)
        pub fn delete_range<R: RangeBounds<isize>>(&mut self, range: R) -> bool {
            let (start, end) = Self::parse_range(range);
            if start > end {
                return false;
            }
            // 全範囲の場合
            if (start, end) == (Self::INF, Self::SUP) {
                let len = self.ranges.len();
                self.ranges.clear();
                self.ranges.insert((Self::INF, Self::INF));
                self.ranges.insert((Self::SUP, Self::SUP));
                return len != self.ranges.len();
            }
            if start == end {
                return self.delete(start);
            }
            while let Some(&(l, r)) = self
                .ranges
                .range((Excluded((start, start)), Excluded((end, end))))
                .next()
            {
                if r < end {
                    self.ranges.remove(&(l, r));
                } else {
                    break;
                }
            }
            let &(ll, l) = self
                .ranges
                .range((Unbounded, Included((start, start))))
                .next_back()
                .unwrap();
            let &(r, rr) = self
                .ranges
                .range((Unbounded, Included((end, end))))
                .next_back()
                .unwrap();
            if l < start && end < r {
                return false;
            }
            if start <= l {
                self.ranges.remove(&(ll, l));
                match (ll < start, end < l) {
                    (false, false) => {}
                    (false, true) => {
                        self.ranges.insert((end.saturating_add(1), l));
                    }
                    (true, false) => {
                        self.ranges.insert((ll, start.saturating_sub(1)));
                    }
                    (true, true) => {
                        self.ranges.insert((ll, start.saturating_sub(1)));
                        self.ranges.insert((end.saturating_add(1), l));
                    }
                }
            }
            if r <= end && end <= rr {
                self.ranges.remove(&(r, rr));
                self.ranges.insert((end.saturating_add(1), rr));
            }
            true
        }
        /// **集合に含まれない**`x`以上で最小の整数を調べる
        pub fn mex(&self, x: isize) -> isize {
            let &(ll, l) = self.ranges.range(..(x + 1, x + 1)).next_back().unwrap();
            if ll <= x && x <= l {
                l + 1
            } else {
                x
            }
        }
    }
}

mod macro_debug {
    #![allow(dead_code)]
    //! デバッグ用マクロ
    /// デバッグ用マクロ
    #[macro_export]
    macro_rules! debug {
        ( $($val:expr),* $(,)* ) => {{
            #[cfg(debug_assertions)]
            eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
        }};
    }
    /// 配列用マクロ
    #[macro_export]
    macro_rules! debug2D {
        ( $array:expr ) => {{
            #![cfg(debug_assertions)]
            eprintln!("{}: ", stringify!($array));
            for row in &$array {
                eprintln!("{:?}", row);
            }
        }};
    }
}
