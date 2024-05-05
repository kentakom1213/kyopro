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
    let mut all = 0;
    // 4方向への累積
    let mut dp = vec![vec![vec![0; W]; H]; 4];
    for i in 0..H {
        for j in 0..W {
            if S[i][j] == '.' {
                all += 1;
                dp[0][i][j] = 1;
                dp[1][i][j] = 1;
                if i > 0 {
                    dp[0][i][j] += dp[0][i - 1][j];
                }
                if j > 0 {
                    dp[1][i][j] += dp[1][i][j - 1];
                }
            }
            // 逆向きの累積和
            let rev_i = H - i - 1;
            let rev_j = W - j - 1;
            if S[rev_i][rev_j] == '.' {
                dp[2][rev_i][rev_j] = 1;
                dp[3][rev_i][rev_j] = 1;
                if rev_i < H - 1 {
                    dp[2][rev_i][rev_j] += dp[2][rev_i + 1][rev_j];
                }
                if rev_j < W - 1 {
                    dp[3][rev_i][rev_j] += dp[3][rev_i][rev_j + 1];
                }
            }
        }
    }

    {
        #![cfg(debug_assertions)]
        for t in &dp {
            for r in t {
                println!("{:?}", r);
            }
            println!();
        }
    }

    // 解への貢献度を計算
    let mut ans = 0;

    for r in 0..H {
        for c in 0..W {
            if S[r][c] == '#' {
                continue;
            }
            let relate = dp[0][r][c] + dp[1][r][c] + dp[2][r][c] + dp[3][r][c] - 3; // (r,c)に照明を設置したときに照らされるマスの数
            let remain = all - relate; // (r,c)に照明を設置したときに照らされないマスの数
            let tmp = (2.mpow(relate).msub(1)).mmul(2.mpow(remain)); // 解への貢献度
            debug!(relate, remain, tmp);
            madd!(ans, tmp)
        }
    }

    println!("{}", ans);
}
