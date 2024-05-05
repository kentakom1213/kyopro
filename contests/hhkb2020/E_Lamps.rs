//                E - Lamps
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/hhkb2020/tasks/hhkb2020_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

/// `a < b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmax {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a < $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
}

/// ## Modint
/// 有限体の実装
pub trait Modint {
    fn val(&self) -> usize;
    fn madd(&self, other: usize) -> usize;
    fn mneg(&self) -> usize;
    fn msub(&self, other: usize) -> usize;
    fn mmul(&self, other: usize) -> usize;
    fn minv(&self) -> usize;
    fn mdiv(&self, other: usize) -> usize;
    fn mpow(&self, other: usize) -> usize;
}

impl Modint for usize {
    fn val(&self) -> usize {
        self % MOD
    }

    fn madd(&self, other: usize) -> usize {
        (self.val() + other.val()).val()
    }

    fn mneg(&self) -> usize {
        (MOD - self.val()).val()
    }

    fn msub(&self, other: usize) -> usize {
        self.madd(other.mneg())
    }

    fn mmul(&self, other: usize) -> usize {
        (self.val() * other.val()).val()
    }

    fn mpow(&self, other: usize) -> usize {
        let (mut a, mut b) = (self.val(), other);
        let mut res = 1;
        while b > 0 {
            if b & 1 == 1 {
                res = res.mmul(a);
            }
            a = a.mmul(a);
            b >>= 1;
        }
        res
    }

    fn minv(&self) -> usize {
        assert!(*self != 0);
        self.mpow(MOD - 2)
    }

    fn mdiv(&self, other: usize) -> usize {
        self.mmul(other.minv())
    }
}

macro_rules! madd {
    ( $a:expr, $b:expr $(,)* ) => {{
        let tmp = ($a).madd($b);
        $a = tmp;
    }};
}

// constant
const MOD: usize = 1_000_000_007;
const INF: usize = 1001001001001001001;

// main
fn main() {
    input! {
        H: usize,
        W: usize,
        S: [Chars; H],
    }

    // '.'のマスの合計
    let X = S
        .iter()
        .map(|s| s.iter().filter(|&&c| c == '.').count())
        .sum::<usize>();

    // cnt[r][c] := マス(r,c)に照明を設置したときに照らされるマスの数
    let cnt = {
        // 縦方向のDP
        let mut dp_x = vec![vec![0_usize; W]; H];

        for i in 0..H {
            for j in 0..W {
                if S[i][j] == '#' {
                    continue;
                }
                dp_x[i][j] = 1;
                if j > 0 {
                    dp_x[i][j] += dp_x[i][j - 1];
                }
            }
            for j in (0..W - 1).rev() {
                if dp_x[i][j] == 0 {
                    continue;
                }
                chmax!(dp_x[i][j], dp_x[i][j + 1],);
            }
        }

        // 横方向のDP
        let mut dp_y = vec![vec![0; W]; H];

        for j in 0..W {
            for i in 0..H {
                if S[i][j] == '#' {
                    continue;
                }
                dp_y[i][j] = 1;
                if i > 0 {
                    dp_y[i][j] += dp_y[i - 1][j];
                }
            }
            for i in (0..H - 1).rev() {
                if dp_y[i][j] == 0 {
                    continue;
                }
                chmax!(dp_y[i][j], dp_y[i + 1][j],);
            }
        }

        // 合計を計算
        let mut cnt = vec![vec![0; W]; H];
        for i in 0..H {
            for j in 0..W {
                cnt[i][j] = (dp_x[i][j] + dp_y[i][j]).saturating_sub(1);
            }
        }

        cnt
    };

    debug!(X);
    debug!(&cnt);

    // 解への貢献度を計算
    let mut ans = 0;

    for r in 0..H {
        for c in 0..W {
            if S[r][c] == '#' {
                continue;
            }
            let relate = cnt[r][c]; // (r,c)に照明を設置したときに照らされるマスの数
            let remain = X - relate; // (r,c)に照明を設置したときに照らされないマスの数
            let tmp = (2.mpow(relate).msub(1)).mmul(2.mpow(remain)); // 解への貢献度
            debug!(relate, remain, tmp);
            madd!(ans, tmp)
        }
    }

    println!("{}", ans);
}
