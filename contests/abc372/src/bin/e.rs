#![allow(non_snake_case)]

use std::{cmp::Reverse, mem};

use cp_library_rs::{data_structure::union_find::UnionFind, debug};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
    }

    let mut uf = UnionFind::new(N + 1);
    let mut nodes = (0..=N).map(|i| vec![Reverse(i)]).collect::<Vec<_>>();

    for _ in 0..Q {
        input!(t: usize);

        match t {
            1 => {
                input! {
                    u: usize,
                    v: usize,
                }

                let u = uf.get_root(u);
                let v = uf.get_root(v);
                uf.unite(u, v);

                if uf.get_root(u) == v {
                    // uがvにマージされたとき
                    let mut removed = mem::replace(&mut nodes[u], vec![]);
                    nodes[v].append(&mut removed);
                    nodes[v].sort();
                    nodes[v].truncate(10);
                } else {
                    // vがuにマージされたとき
                    let mut removed = mem::replace(&mut nodes[v], vec![]);
                    nodes[u].append(&mut removed);
                    nodes[u].sort();
                    nodes[u].truncate(10);
                }

                debug!(nodes);
            }
            2 => {
                input! {
                    v: usize,
                    k: Usize1,
                }

                let v = uf.get_root(v);

                if let Some(&Reverse(res)) = nodes[v].get(k) {
                    println!("{}", res);
                } else {
                    println!("-1");
                }
            }
            _ => (),
        }
    }
}
