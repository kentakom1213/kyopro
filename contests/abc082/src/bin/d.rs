// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use std::mem::swap;

// imports
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

/// ## ランレングス圧縮
/// - スライスからエンコードを行う
pub fn run_length_encode<T>(arr: &[T]) -> Vec<(T, usize)>
where
    T: PartialEq + Copy,
{
    let mut res = vec![];
    let mut cur = arr[0];
    let mut cnt = 1;
    for &val in &arr[1..] {
        if val == cur {
            cnt += 1;
        } else {
            res.push((cur, cnt));
            cur = val;
            cnt = 1;
        }
    }
    let last_elem = *arr.last().unwrap();
    res.push((last_elem, cnt));
    res
}
/// ## ランレングス圧縮 (from Iterator)
/// - イテレータからエンコードを行う
pub fn run_length_encode_from<T, I>(mut itr: I) -> Vec<(T, usize)>
where
    T: PartialEq,
    I: Iterator<Item = T>,
{
    let mut res = vec![];
    let mut cur = itr.next().unwrap();
    let mut cnt = 1;
    for val in itr {
        if val == cur {
            cnt += 1;
        } else {
            res.push((cur, cnt));
            cur = val;
            cnt = 1;
        }
    }
    res.push((cur, cnt));
    res
}

fn main() {
    input! {
        S: String,
        X: isize,
        Y: isize,
    }

    // 幅の最大値
    let OFFSET = S.chars().filter(|&c| c == 'F').count();
    let WIDTH = (OFFSET + 2) * 2;

    // ランレングス圧縮
    let rlt = run_length_encode_from(S.chars());
    let N = rlt.len();

    let can_x = {
        // cur[x] := i番目までみたとき、位置xにいることは可能か
        let mut cur = vec![false; WIDTH];
        let mut nxt = vec![false; WIDTH];
        cur[OFFSET] = true;

        let mut cnt = 0;
        for i in 0..N {
            let (c, n) = rlt[i];
            // 直進（向きが水平のときのみ）
            if c == 'F' && cnt % 2 == 0 {
                for x in 0..WIDTH {
                    if !cur[x] {
                        continue;
                    }
                    if i > 0 && x >= n {
                        nxt[x - n] |= true;
                    }
                    if x + n < WIDTH {
                        nxt[x + n] |= true;
                    }
                }
            } else {
                // 下に伝搬
                for x in 0..WIDTH {
                    nxt[x] = cur[x];
                }
                if c == 'T' {
                    cnt += n;
                }
            }

            debug!(c, n, &nxt);

            swap(&mut cur, &mut nxt);
            nxt.fill(false);
        }
        cur
    };

    debug!(&can_x);

    let can_y = {
        // cur[y] := i番目までみたとき、位置yにいることは可能か
        let mut cur = vec![false; WIDTH];
        let mut nxt = vec![false; WIDTH];
        cur[OFFSET] = true;

        let mut cnt = 0;
        for i in 0..N {
            let (c, n) = rlt[i];
            // 直進（向きが水平のときのみ）
            if c == 'F' && cnt % 2 == 1 {
                for y in 0..WIDTH {
                    if !cur[y] {
                        continue;
                    }
                    if y >= n {
                        nxt[y - n] |= true;
                    }
                    if y + n < WIDTH {
                        nxt[y + n] |= true;
                    }
                }
            } else {
                // 下に伝搬
                for y in 0..WIDTH {
                    nxt[y] = cur[y];
                }
                if c == 'T' {
                    cnt += n;
                }
            }

            debug!(c, n, &nxt);

            swap(&mut cur, &mut nxt);
            nxt.fill(false);
        }
        cur
    };

    debug!(&can_y);

    // 判定
    let X = OFFSET as isize + X;
    let Y = OFFSET as isize + Y;

    if X < 0 || WIDTH < X as usize || Y < 0 || WIDTH < Y as usize {
        println!("No");
        return;
    }

    if can_x[X as usize] && can_y[Y as usize] {
        println!("Yes");
    } else {
        println!("No");
    }
}
