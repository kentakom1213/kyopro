//             G - Triple Index
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc293/tasks/abc293_g
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
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

pub trait UsizeTools {
    fn abs_diff(&self, other: Self) -> Self;
    fn sqrt(&self) -> Self;
}

impl UsizeTools for usize {
    fn abs_diff(&self, other: Self) -> Self {
        if *self > other {
            *self - other
        } else {
            other - *self
        }
    }

    /// ## sqrt
    /// x^2がNを超えない最大のxを求める
    /// - 計算量：O(log(N))
    fn sqrt(&self) -> Self {
        let (mut ok, mut ng) = (0_usize, 1001001001001001001);
        while (ng - ok) > 1 {
            let m = (ok + ng) / 2;
            if m.saturating_mul(m) <= *self {
                ok = m;
            } else {
                ng = m;
            }
        }
        ok
    }
}

/// ## Mo's Algorithm
/// 🐮🐮🐮
fn main() {
    input! {
        N: usize,
        Q: usize,
        A: [usize; N],
        queries: [(Usize1, usize); Q],
    }

    let B_SIZ = N.sqrt(); // バケットのサイズ

    // クエリのソート
    let sorted_query = queries.iter().enumerate().sorted_by_key(|(i, (l, r))| (l / B_SIZ, r));

    let mut res = vec![0; Q];

    // 現在の区間における答え
    let mut ans = 0_usize;

    // ### 区間 ###
    // 現在の位置
    let (mut nl, mut nr) = (0, 0);

    // 区間におけるxの個数を管理
    let mut cnt = vec![0; 200200];

    // ### 関数 ###
    // nC3を返す
    let comb3 = |n: usize| -> usize {
        if n <= 1 { return 0; }
        n * (n - 1) * (n - 2) / 6
    };

    // 区間にxを追加
    let add = |x: usize, ans: &mut usize, cnt: &mut [usize]| {
        *ans -= comb3(cnt[x]);
        cnt[x] += 1;
        *ans += comb3(cnt[x]);
    };

    // 区間からxを削除
    let del = |x: usize, ans: &mut usize, cnt: &mut [usize]| {
        *ans -= comb3(cnt[x]);
        cnt[x] -= 1;
        *ans += comb3(cnt[x]);
    };

    for (i, &(l, r)) in sorted_query {
        while nl > l {
            nl -= 1;
            add(A[nl], &mut ans, &mut cnt);
        }
        while nr < r {
            add(A[nr], &mut ans, &mut cnt);
            nr += 1;
        }
        while nl < l {
            del(A[nl], &mut ans, &mut cnt);
            nl += 1;
        }
        while nr > r {
            nr -= 1;
            del(A[nr], &mut ans, &mut cnt);
        }
        // 答えを保存
        res[i] = ans;
    }

    // 出力
    println!("{}", res.iter().join("\n"));
}
