#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::operation::Add, data_structure::segment_tree::SegmentTree, debug,
    utils::show_tree::ShowBinaryTree,
};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        S: String,
        LR: [(Usize1, usize); Q],
    }

    let seg1 = S
        .chars()
        .map(|x| (x == '1') as usize)
        .collect::<SegmentTree<Add<usize>>>();

    let seg2 = S
        .chars()
        .map(|x| (x == '2') as usize)
        .collect::<SegmentTree<Add<usize>>>();

    let mut T = vec![0; N + 1];
    for (i, c) in S.chars().enumerate() {
        T[i + 1] = T[i];
        if c == '/' {
            T[i + 1] += 1;
        }
    }

    seg1.print_as_binary_tree();
    seg2.print_as_binary_tree();
    debug!(T);

    // 長さ 2k+1 の '1..1/2..2' が存在するか
    let isok = |l: usize, r: usize, k: usize| -> bool {
        let (_, ll) = seg1.max_right(l, |x| x < k);
        let (_, rr) = seg2.min_left(r, |x| x < k);

        debug!(l, r, k, ll, rr);

        // ll..rr の間に / が存在するか
        T[ll] < T[rr]
    };

    for (l, r) in LR {
        // l..rの間に'/'が存在しない → 0
        if T[r] - T[l] == 0 {
            println!("0");
            continue;
        }

        let mut ok = 0;
        let mut ng = N;

        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            if isok(l, r, mid) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        let ans = ok * 2 + 1;
        println!("{ans}");
    }
}
