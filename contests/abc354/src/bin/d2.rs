#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        A: isize,
        B: isize,
        C: isize,
        D: isize,
    }

    // Bを0,1のどちらかに固定
    let (B, D) = {
        let pb = B.rem_euclid(2);
        let pd = D - B + pb;
        (pb as usize, pd as usize)
    };

    // Aを0,1,2,3のどれかに固定
    let (A, C) = {
        let pa = A.rem_euclid(4);
        let pc = C - A + pa;
        (pa as usize, pc as usize)
    };

    debug!(A, B, C, D);

    // パターンの数
    let dx = C - A;
    let dy = D - B;

    let rep_x = dx / 4;
    let rep_y = dy / 2;

    // 繰り返し回数
    let rep = (dx / 4) * (dy / 2);

    let N_centor = rep * 8;

    debug!(rep);

    // あまりの個数
    let ma = A % 4;
    let mc = C % 4;
    let mb = B % 2;
    let md = D % 2;

    // 両端の数え上げ
    let N_lr = {
        let ma = A % 4;
        let mut mc = C % 4;

        if ma > mc {
            mc += 4;
        }

        0
    };

    let N_td = {
        0
    };

    // 角の数え上げ
    let N_corner = {
        let ma = A % 4;
        let mut mc = C % 4;
        let mb = B % 2;
        let mut md = D % 2;

        if ma > mc {
            mc += 4;
        }
        if mb > md {
            md += 2;
        }

        let mut sum = 0;

        for x in ma..mc {
            for y in mb..md {
                sum += CORNER[x][y];
            }
        }

        sum
    };

    debug!(N_corner);

    let ans = N_centor + N_lr + N_td + N_corner;

    println!("{ans}");
}

const PTN_X: [usize; 4] = [3, 3, 1, 1];
const PTN_Y: [usize; 2] = [4, 4];
const CORNER: [[usize; 2]; 4] = [[2, 1], [1, 2], [0, 1], [1, 0]];

const _INF: usize = 1001001001001001001;

mod macro_debug {
    #![allow(dead_code)]
    //! デバッグ用マクロ
    /// デバッグ用マクロ
    #[macro_export]
    macro_rules! debug {
        ( $($val:expr),* $(,)* ) => {{
            #[cfg(debug_assertions)]
            eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
        }};
    }
    /// 配列用マクロ
    #[macro_export]
    macro_rules! debug2D {
        ( $array:expr ) => {{
            #![cfg(debug_assertions)]
            eprintln!("{}: ", stringify!($array));
            for row in &$array {
                eprintln!("{:?}", row);
            }
        }};
    }
}
