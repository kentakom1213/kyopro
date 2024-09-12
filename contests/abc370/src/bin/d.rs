#![allow(non_snake_case)]
use cp_library_rs::{data_structure::mex_set::MexSet, debug, debug2D};
use itertools::iproduct;
use proconio::{input, marker::Isize1};
use rustc_hash::FxHashMap;

fn main() {
    input! {
        H: usize,
        W: usize,
        Q: usize,
    }

    let mut broken = vec![vec![0; W]; H];
    let mut rows = FxHashMap::<isize, MexSet>::default();
    let mut cols = FxHashMap::<isize, MexSet>::default();

    for _ in 0..Q {
        input! {
            r: Isize1,
            c: Isize1,
        }

        if broken[r as usize][c as usize] == 0 {
            broken[r as usize][c as usize] = 1;
            rows.entry(r).or_insert_with(MexSet::new).insert(c);
            cols.entry(c).or_insert_with(MexSet::new).insert(r);
        } else {
            let row = rows.get_mut(&r).unwrap();
            let left = row.mex_rev(c);

            if left >= 0 {
                broken[r as usize][left as usize] = 1;
                row.insert(left);
                cols.entry(left).or_insert_with(MexSet::new).insert(r);
            }

            let right = row.mex(c);

            if right < W as isize {
                broken[r as usize][right as usize] = 1;
                row.insert(right);
                cols.entry(right).or_insert_with(MexSet::new).insert(r);
            }

            debug!(left, right);

            let col = cols.get_mut(&c).unwrap();
            let up = col.mex_rev(r);

            if up >= 0 {
                broken[up as usize][c as usize] = 1;
                col.insert(up);
                rows.entry(up).or_insert_with(MexSet::new).insert(c);
            }

            let down = col.mex(r);

            if down < H as isize {
                broken[down as usize][c as usize] = 1;
                col.insert(down);
                rows.entry(down).or_insert_with(MexSet::new).insert(c);
            }

            debug!(up, down);
        }

        debug2D!(broken);
    }

    let ans = iproduct!(0..H, 0..W)
        .filter(|&(r, c)| broken[r][c] == 0)
        .count();

    println!("{}", ans);
}
