#![allow(non_snake_case)]

use cp_library_rs::{debug, graph::lca_doubling::LCA, utils::consts::INF};
use proconio::input;

/// 観察:
/// - mex(P) = k になるようなパス P を k に関して昇順に求めていく
/// - このようなパス P を包含するようなパスの個数を数え上げれば良い（寄与）
fn main() {
    input! {
        N: usize,
        UV: [(usize, usize); N - 1]
    }

    let G = UV.iter().fold(vec![vec![]; N], |mut g, &(u, v)| {
        g[u].push(v);
        g[v].push(u);
        g
    });
    let lca = LCA::new(&G, 0);
    let path_contains = |l, r, x| lca.dist(l, x) + lca.dist(x, r) == lca.dist(l, r);

    let mut subtree = vec![0; N];
    dfs(0, INF, &mut subtree, &G);
    debug!(subtree);

    // 部分木のサイズ: u→v の u側の部分木のサイズ
    let subtree_size = |u: usize, v: usize| {
        let w = lca.kth_on_path(u, v, 1).unwrap();

        if lca.parent(w).is_some_and(|p| p == u) {
            N - subtree[w]
        } else {
            subtree[u]
        }
    };

    let mut ans = 0;

    // {0} を通るパスの数
    // 0 を始点とするパス
    ans += N;

    // 0 をまたぐパス
    let mut s = 0;
    for &v in &G[0] {
        ans += s * subtree[v];
        s += subtree[v];
    }

    debug!(ans);

    // 左右に伸ばしていく
    let (mut l, mut r) = (0, 0);

    for k in 1..N {
        // l,r の更新
        if path_contains(l, k, r) {
            r = k;
        } else if path_contains(k, r, l) {
            l = k;
        } else if path_contains(l, r, k) {
            // 何もしない
        } else {
            break;
        }

        // l,r を端点とするパスの数を求める
        let lsize = subtree_size(l, r);
        let rsize = subtree_size(r, l);

        debug!(l, r, lsize, rsize);

        ans += lsize * rsize;
    }

    println!("{ans}");
}

fn dfs(u: usize, p: usize, subtree: &mut [usize], G: &[Vec<usize>]) {
    for &v in &G[u] {
        if p == v {
            continue;
        }
        dfs(v, u, subtree, G);
        subtree[u] += subtree[v];
    }
    subtree[u] += 1;
}
