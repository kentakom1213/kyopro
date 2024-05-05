#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}
macro_rules! debug2D {
    ( $array:expr ) => {{
        #![cfg(debug_assertions)]
        eprintln!("{}: ", stringify!($array));
        for row in &$array {
            eprintln!("{:?}", row);
        }
    }};
}

use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

fn main() {
    input! {
        N: usize,
        X: isize,
        Y: isize,
        A: [isize; N],
    }

    // X方向，Y方向に分割
    let (DX, DY) = A
        .iter()
        .enumerate()
        .fold((vec![], vec![]), |(mut X, mut Y), (i, &a)| {
            if i % 2 == 0 {
                Y.push(a);
            } else {
                X.push(a);
            }
            (X, Y)
        });

    debug!(DX, DY);

    let route_x = find_route(&DX, X);
    let route_y = find_route(&DY, Y);

    let (Some(route_x), Some(route_y)) = (route_x, route_y) else {
        println!("No");
        return;
    };
}

fn find_route(D: &[isize], T: isize) -> Option<Vec<isize>> {
    // 半分全列挙で，実現可能性を判定
    let mid = D.len() / 2;
    let (L, R) = (&D[..mid], &D[mid..]);
    let (l, r) = (L.len(), R.len());

    // 右側を前列挙
    

    todo!()
}
