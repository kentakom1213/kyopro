#![allow(non_snake_case)]

use cp_library_rs::{
    utils::{consts::IINF, index_isize::IndexIsize},
};
use proconio::input;

fn main() {
    input! {
        N: usize,
        D: isize,
        XY: [(isize, isize); N]
    }

    let (X, Y): (Vec<isize>, Vec<isize>) = XY.into_iter().unzip();

    // f[x] := Σ |x - x_i|
    let mut f = calc(X);

    // g[y] := Σ |y - y_i|
    let mut g = calc(Y);

    // 問題: f[i] + g[j] <= D を満たすような (i,j) の組は何通り存在するか
    // → 尺取り法で解くことができる
    f.sort();
    g.sort();

    let mut ans = 0;
    let mut j = 0;

    for &x in f.iter().rev() {
        while j < 2 * UM && x + g[j] <= D {
            j += 1;
        }
        ans += j;
    }

    println!("{ans}");
}

fn calc(mut xs: Vec<isize>) -> Vec<isize> {
    xs.sort();
    let mut res = vec![IINF; 2 * UM + 10];
    let N = xs.len() as isize;
    *res.iget_mut(-M) = xs.iter().sum::<isize>() + N * M;

    // xより小さい要素の個数
    let mut i = 0;

    for x in -M + 1..=M {
        while i < N && xs[i as usize] < x {
            i += 1;
        }
        *res.iget_mut(x) = res.iget(x - 1) + i - (N - i);
    }

    res
}

const M: isize = 2001001;
const UM: usize = M as usize;
