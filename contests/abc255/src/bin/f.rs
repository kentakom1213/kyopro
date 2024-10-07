#![allow(non_snake_case)]

use cp_library_rs::{
    debug, debug2D,
    utils::consts::{INF, NEG1},
};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        P: [Usize1; N],
        I: [Usize1; N],
    }

    // idx[i] := 頂点iの(Pでのインデックス, Iでのインデックス)
    let idxs = (0..N).fold(vec![(INF, INF); N], |mut idx, i| {
        idx[P[i]].0 = i;
        idx[I[i]].1 = i;
        idx
    });

    debug!(idxs);

    if P[0] != 0 {
        println!("-1");
        return;
    }

    let Some(res) = build_tree(0, (0, N), vec![(NEG1, NEG1); N], &P, &I, &idxs) else {
        println!("-1");
        return;
    };

    debug2D!(res);

    for (l, r) in res {
        println!("{} {}", l.wrapping_add(1), r.wrapping_add(1));
    }
}

/// 再帰的に2分木を構築する
fn build_tree(
    root: usize,
    (l, r): (usize, usize),
    mut res: Vec<(usize, usize)>,
    P: &[usize],
    I: &[usize],
    idxs: &[(usize, usize)],
) -> Option<Vec<(usize, usize)>> {
    let (root_p, root_i) = idxs[root];

    // 左の子の数
    let l_cnt = root_i - l;
    // 右の子の数
    let r_cnt = r - (root_i + 1);

    debug!(l_cnt, r_cnt);

    if l_cnt > 0 {
        // 左の子
        let &l_ch = P.get(root_p + 1).filter(|&&l_ch| {
            let i = idxs[l_ch].1;
            l <= i && i < l + l_cnt
        })?;

        res[root].0 = l_ch;

        // 再帰的に構築
        res = build_tree(l_ch, (l, l + l_cnt), res, P, I, idxs)?;
    }

    if r_cnt > 0 {
        // 右の子
        let &r_ch = P.get(root_p + 1 + l_cnt).filter(|&&r_ch| {
            let i = idxs[r_ch].1;
            root_i + 1 <= i && i < r
        })?;

        res[root].1 = r_ch;

        // 再帰的に構築
        res = build_tree(r_ch, (root_i + 1, r), res, P, I, idxs)?;
    }

    Some(res)
}
