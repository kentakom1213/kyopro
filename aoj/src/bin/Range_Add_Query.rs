//             Range Add Query
// ----------------------------------------
// 問題
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_E&lang=ja
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// [Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)
macro_rules! get {
    ($t:ty) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
        }
    };
    ($($t:ty),*) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
        }
    };
    ($t:ty ; $n:expr) => {
        (0..$n).map(|_|
            get!($t)
        ).collect::<Vec<_>>()
    };
    ($($t:ty),* ; $n:expr) => {
        (0..$n).map(|_|
            get!($($t),*)
        ).collect::<Vec<_>>()
    };
    ($t:ty ;;) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
        }
    };
    ($t:ty ;; $n:expr) => {
        (0..$n).map(|_|
            get!($t ;;)
        ).collect::<Vec<_>>()
    };
}

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// データ型
type T = isize;

// 単位元
const I: T = 0;

/// ## LazySegmentTree
/// - 遅延セグメント木
/// - 区間加算、区間取得をサポートする
/// - 対応している型はisizeのみ（ジェネリクスなし）
#[derive(Debug)]
pub struct LazySegmentTree {
    size: usize,
    offset: usize,
    data: Vec<T>,
    lazy: Vec<T>,
}

impl LazySegmentTree {
    /// 新規作成
    pub fn new(n: usize) -> Self {
        let offset = n.next_power_of_two();
        Self {
            size: n,
            offset,
            data: vec![I; offset << 1],
            lazy: vec![I; offset << 1],
        }
    }

    /// 遅延値を評価
    fn eval(&mut self, idx: usize, len: usize) {
        // 葉でなければ子に伝搬
        if idx < self.offset {
            self.lazy[idx * 2] += self.lazy[idx];
            self.lazy[idx * 2 + 1] += self.lazy[idx];
        }
        // 自身を更新
        self.data[idx] += self.lazy[idx] * len as isize;
        self.lazy[idx] = I;
    }

    /// 区間加算
    /// - [left, right)
    pub fn set_range(&mut self, left: usize, right: usize, val: T) {
        self.set_range_sub(left, right, val, 0, self.offset, 1);
    }

    fn set_range_sub(
        &mut self,
        left: usize,
        right: usize,
        val: T,
        begin: usize,
        end: usize,
        idx: usize,
    ) {
        // 遅延値を評価
        self.eval(idx, end - begin);
        // 区間を内包するとき
        if left <= begin && end <= right {
            self.lazy[idx] += val;
            self.eval(idx, end - begin);
        }
        // 区間が重なるとき
        else if left < end && begin < right {
            let mid = (begin + end) / 2;
            // 左の子を更新
            self.set_range_sub(left, right, val, begin, mid, idx * 2);
            // 右の子を更新
            self.set_range_sub(left, right, val, mid, end, idx * 2 + 1);
            // 値を更新
            self.data[idx] = self.data[idx * 2] + self.data[idx * 2 + 1];
        }
    }

    /// 区間取得
    /// - 再帰実装
    /// - [left, right)
    pub fn get_range(&mut self, left: usize, right: usize) -> T {
        self.get_range_sub(left, right, 0, self.offset, 1)
    }

    fn get_range_sub(
        &mut self,
        left: usize,
        right: usize,
        begin: usize,
        end: usize,
        idx: usize,
    ) -> T {
        // 遅延値を評価
        self.eval(idx, end - begin);
        // 区間を含まない
        if end <= left || right <= begin {
            I
        }
        // 区間を包含する
        else if left <= begin && end <= right {
            self.data[idx]
        }
        // 区間が重なる
        else {
            let mid = (begin + end) / 2;
            let l_val = self.get_range_sub(left, right, begin, mid, idx * 2);
            let r_val = self.get_range_sub(left, right, mid, end, idx * 2 + 1);
            l_val + r_val
        }
    }
}

// main
fn main() {
    let (N, Q) = get!(usize, usize);

    // 遅延セグメント木
    let mut seg = LazySegmentTree::new(N);

    // クエリの処理
    for _ in 0..Q {
        let query = get!(usize;;);

        if let &[1, i] = &query[..2] {
            // 取得
            let res = seg.get_range(i - 1, i);
            println!("{}", res);
        }
        else if let &[0, s, t, x] = &query[..4] {
            // 区間加算
            seg.set_range(s - 1, t, x as isize);
        }

        debug!(&seg);
    }
}
