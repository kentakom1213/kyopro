#![allow(non_snake_case)]

use cp_library_rs::{data_structure::union_find::UnionFind, debug};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
        mut UVW: [(Usize1, Usize1, usize); M],
        A: [Usize1; K],
        B: [Usize1; K]
    }

    let mut cnt_a = vec![0; N];
    let mut cnt_b = vec![0; N];

    for (&a, &b) in A.iter().zip(&B) {
        cnt_a[a] += 1;
        cnt_b[b] += 1;
    }

    debug!(cnt_a, cnt_b);

    let mut ans = 0;
    let mut uf = UnionFind::new(N);

    UVW.sort_by_key(|&(_, _, w)| w);

    for &(u, v, w) in &UVW {
        let mut ru = uf.root(u);
        let mut rv = uf.root(v);

        if let Some(p) = uf.unite(ru, rv) {
            // ruを親に
            if p == rv {
                (ru, rv) = (rv, ru);
            }

            cnt_a[ru] += cnt_a[rv];
            cnt_b[ru] += cnt_b[rv];

            // マッチする数
            let m = cnt_a[ru].min(cnt_b[ru]);

            ans += w * m;

            // マッチした分削除
            cnt_a[ru] -= m;
            cnt_b[ru] -= m;
        }
    }

    println!("{ans}");
}
