#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::monoid::examples::Add, data_structure::segment_tree::SegmentTree, debug,
    get, graph::euler_tour::EulerTour,
};

fn main() {
    let N = get!(usize);

    let mut T = EulerTour::new(N);

    for u in 0..N {
        let ch = get!(usize;;);

        for &v in &ch[1..] {
            T.add_edge(u, v);
        }
    }

    T.build(0);

    debug!(T);

    // セグメント木
    let mut seg = SegmentTree::<Add<isize>>::new(2 * N);

    // クエリ処理
    let Q = get!(usize);

    for _ in 0..Q {
        let q = get!(usize;;);

        if let &[0, v, w] = &q[..] {
            let f = T.in_[v];
            let b = T.out[v];
            *seg.get_mut(f).unwrap() += w as isize;
            *seg.get_mut(b).unwrap() -= w as isize;
        }

        if let &[1, u] = &q[..] {
            let b = T.out[u];
            let res = seg.get_range(..b);

            println!("{}", res);
        }
    }
}
