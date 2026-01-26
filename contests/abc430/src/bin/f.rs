#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::operation::Add, data_structure::dual_segment_tree::DualSegmentTree, debug,
};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        T: usize
    }

    for _ in 0..T {
        solve();
    }
}

fn solve() {
    input! {
        N: usize,
        S: Chars
    }

    // i より左,右 にあることが確定している要素の数
    let mut nl: Vec<usize> = vec![0; N];
    let mut nr: Vec<usize> = vec![0; N];

    let mut lcnt = 0;
    let mut rcnt = 0;

    for i in 0..N - 1 {
        nl[i] += rcnt;
        nr[i] += lcnt;

        if S[i] == 'L' {
            lcnt += 1;
            rcnt = 0;
        } else {
            lcnt = 0;
            rcnt += 1;
        }
    }
    nl[N - 1] += rcnt;
    nr[N - 1] += lcnt;

    debug!(nl, nr);

    let mut lcnt = 0;
    let mut rcnt = 0;

    for i in (1..N).rev() {
        nl[i] += lcnt;
        nr[i] += rcnt;

        if S[i - 1] == 'L' {
            lcnt += 1;
            rcnt = 0;
        } else {
            lcnt = 0;
            rcnt += 1;
        }
    }
    nl[0] += lcnt;
    nr[0] += rcnt;

    debug!(nl, nr);

    // 左右の数に応じて区間加算
    let mut seg = DualSegmentTree::<Add<usize>>::new(N);

    for i in 0..N {
        let l = nl[i];
        let r = N - nr[i];
        seg.apply_range(l..r, 1);
    }

    debug!(seg);

    for i in 0..N {
        print!("{} ", seg.get_point(i));
    }
    println!()
}
