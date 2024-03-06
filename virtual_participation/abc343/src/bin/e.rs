#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use itertools::{iproduct, izip, Itertools};
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Usize1},
};

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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        V1: usize,
        V2: usize,
        V3: usize,
    }

    // 7x7x7格子上の点
    let box7: Vec<Pos> = iproduct!(0..=7, 0..=7, 0..=7).collect_vec();
    let C = box7.len();

    for i in 0..C {
        for j in i..C {
            for k in j..C {
                let (u, v, w) = (box7[i], box7[j], box7[k]);
                let vs = get_vols(u, v, w);
                debug!(u, v, w, vs);
                if vs == (V1, V2, V3) {
                    println!("Yes");
                    print!("{} {} {} ", u.0, u.1, u.2);
                    print!("{} {} {} ", v.0, v.1, v.2);
                    println!("{} {} {}", w.0, w.1, w.2);
                    return;
                }
            }
        }
    }

    println!("No");
}

type Pos = (usize, usize, usize);

fn add7((a, b, c): Pos) -> Pos {
    (a + 7, b + 7, c + 7)
}

fn get_vols(u: Pos, v: Pos, w: Pos) -> Pos {
    let U = (u, add7(u));
    let V = (v, add7(v));
    let W = (w, add7(w));

    // 共通部分を求める
    let uv = get_intersect(U, V);
    let vw = get_intersect(V, W);
    let wu = get_intersect(W, U);
    let uvw = get_intersect(get_intersect(U, V), W);

    // 共通部分の体積を求める
    let V_uv = get_vol(uv);
    let V_vw = get_vol(vw);
    let V_wu = get_vol(wu);
    let V_uvw = get_vol(uvw);

    // let V1 =
    // let V2 =
    let V3 = V_uvw;
    let V2 = V_uv + V_vw + V_wu - 3 * V3;
    let V1 = 3 * (7 * 7 * 7) - 2 * V2 - 3 * V3;

    (V1, V2, V3)
}

/// 直方体の体積を求める
fn get_vol(((sx, sy, sz), (ex, ey, ez)): (Pos, Pos)) -> usize {
    let hx = ex.abs_diff(sx);
    let hy = ey.abs_diff(sy);
    let hz = ez.abs_diff(sz);
    hx * hy * hz
}

/// 2つの長方形の共通部分を求める
fn get_intersect(u: (Pos, Pos), v: (Pos, Pos)) -> (Pos, Pos) {
    let (us, ue) = u;
    let (vs, ve) = v;
    // 始点
    let xs = us.0.max(vs.0);
    let ys = us.1.max(vs.1);
    let zs = us.2.max(vs.2);
    let s = (xs, ys, zs);
    // 終点
    let xe = ue.0.min(ve.0);
    let ye = ue.1.min(ve.1);
    let ze = ue.2.min(ve.2);
    let e = (xe, ye, ze);

    (s, e)
}

#[cfg(test)]
mod test_abc343_e {
    use crate::{add7, get_intersect, get_vol};

    #[test]
    fn test_get_intersection() {
        // かさなりなし
        let u = (0, 0, 0);
        let v = (0, 7, 0);
        let U = (u, add7(u));
        let V = (v, add7(v));
        debug!(U, V);
        let uv = get_intersect(U, V);
        debug!(uv);

        assert_eq!(get_vol(uv), 0);

        // 1だけ
        let u = (0, 0, 0);
        let v = (6, 6, 6);
        let U = (u, add7(u));
        let V = (v, add7(v));
        debug!(U, V);
        let uv = get_intersect(U, V);
        debug!(uv);

        assert_eq!(get_vol(uv), 1);
    }
}
