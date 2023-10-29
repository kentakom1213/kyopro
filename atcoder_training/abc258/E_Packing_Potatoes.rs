//           E - Packing Potatoes
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc258/tasks/abc258_e
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

fn debug_2d<T: std::fmt::Debug>(array: &Vec<Vec<T>>) {
    #[cfg(debug_assertions)]
    for row in array {
        println!("{:?}", row);
    }
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

// main
#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        X: usize,
        W: [usize; N],
        queries: [usize; Q],
    }

    // 累積和
    let mut S = vec![0; 2 * N + 1];
    for i in 0..2 * N {
        S[i + 1] = S[i] + W[i % N];
    }
    let total = S[N];

    debug!(&S, total);

    // to[i] := iから始まる箱のが閉じた後、次に始まるインデックス
    let mut to = vec![0; N];

    for i in 0..N {
        let rem = X % total;
        let (mut ng, mut ok) = (i.wrapping_sub(1), 2 * N);
        while ok.wrapping_sub(ng) > 1 {
            let mid = ng.wrapping_add(ok) / 2;
            if S[mid] - S[i] >= rem {
                ok = mid;
            } else {
                ng = mid;
            }
            to[i] = ok % N;
        }
    }

    // ダブリング配列
    let mut double = vec![vec![0; N]; 60];
    double[0] = to.clone();

    for i in 1..60 {
        for j in 0..N {
            double[i][j] = double[i - 1][double[i - 1][j]];
        }
    }

    debug_2d(&double);

    // 0からx回遷移した先を返す関数
    let transition = |x: usize| -> usize {
        let mut cur = 0;
        for i in (0..60).rev() {
            if x >> i & 1 == 1 {
                cur = double[i][cur];
            }
        }
        cur
    };

    for &q in &queries {
        let cur = transition(q - 1);

        // 何周するか
        let mut ans = X / total * N;
        if cur < to[cur] {
            ans += to[cur] - cur;
        } else {
            ans += N + to[cur] - cur;
        }

        // ちょうど割り切れる時
        if X % total == 0 {
            ans = X / total * N;
        }

        println!("{}", ans);
    }
}
