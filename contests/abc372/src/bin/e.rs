use std::cmp::Reverse;

use cp_library_rs::{
    data_structure::{indexedset::IndexedSet, union_find::UnionFind},
    debug,
};
use proconio::{input, marker::Usize1};

#[allow(non_snake_case)]

fn main() {
    input! {
        N: usize,
        Q: usize,
    }

    let mut uf = UnionFind::new(N + 1);
    let mut nodes = (0..=N)
        .map(|i| IndexedSet::from_iter([Reverse(i)]))
        .collect::<Vec<_>>();

    for _ in 0..Q {
        input!(t: u8);

        match t {
            1 => {
                input! {
                    mut u: usize,
                    mut v: usize,
                }

                u = uf.root(u);
                v = uf.root(v);

                let Some(p) = uf.unite(u, v) else {
                    continue;
                };

                if p == u {
                    // uにマージされた場合
                    let removed = std::mem::replace(&mut nodes[v], IndexedSet::new());
                    for &x in &removed {
                        nodes[u].insert(x);
                    }
                } else {
                    // vにマージされた場合
                    let removed = std::mem::replace(&mut nodes[u], IndexedSet::new());
                    for &x in &removed {
                        nodes[v].insert(x);
                    }
                }

                debug!(nodes);
            }
            2 => {
                input! {
                    mut v: usize,
                    k: Usize1,
                }

                v = uf.root(v);

                if let Some(&Reverse(x)) = nodes[v].get_by_index(k) {
                    println!("{x}")
                } else {
                    println!("-1");
                }
            }
            _ => (),
        }
    }
}
