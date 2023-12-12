// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::Itertools;
use num_integer::Roots;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

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

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
        q: usize,
        queries: [(Usize1, usize); q]
    }

    // Mo's Algorithm

    let B_SIZ = n.sqrt(); // バケットのサイズ

    // クエリのソート
    let sorted_query = queries
        .iter()
        .enumerate()
        .sorted_by_key(|(i, (l, r))| (l / B_SIZ, r));

    let mut res = vec![0; q];

    // 現在の区間における答え
    let mut ans = 0_usize;

    // ### 区間 ###
    // 現在の位置
    let (mut nl, mut nr) = (0, 0);

    // 区間におけるxの個数を管理
    let mut cnt = vec![0; n];

    // 区間にxを追加
    let add = |x: usize, ans: &mut usize, cnt: &mut [usize]| {
        cnt[x] += 1;
        // ペアが新しく作れる
        if cnt[x] % 2 == 0 {
            *ans += 1;
        }
    };

    // 区間からxを削除
    let del = |x: usize, ans: &mut usize, cnt: &mut [usize]| {
        cnt[x] -= 1;
        // ペアを削除
        if cnt[x] % 2 == 1 {
            *ans -= 1;
        }
    };

    for (i, &(l, r)) in sorted_query {
        while nl > l {
            nl -= 1;
            add(a[nl], &mut ans, &mut cnt);
        }
        while nr < r {
            add(a[nr], &mut ans, &mut cnt);
            nr += 1;
        }
        while nl < l {
            del(a[nl], &mut ans, &mut cnt);
            nl += 1;
        }
        while nr > r {
            nr -= 1;
            del(a[nr], &mut ans, &mut cnt);
        }
        // 答えを保存
        res[i] = ans;
        debug!((l, r), cnt);
    }

    // 出力
    println!("{}", res.iter().join("\n"));
}
