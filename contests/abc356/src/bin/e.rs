#![allow(non_snake_case)]

use proconio::input;
use superslice::Ext;

use crate::run_length::run_length_encode;

fn main() {
    input! {
        N: usize,
        mut A: [usize; N]
    }

    A.sort();

    // ランレングス圧縮
    let rle = run_length_encode(&A);
    // 累積和
    let mut S = vec![0; rle.len() + 1];
    for (i, &(_, cnt)) in rle.iter().enumerate() {
        S[i + 1] = S[i] + cnt;
    }

    debug!(rle);
    debug!(S);

    let mut ans = 0;

    for (i, &(a, c)) in rle.iter().enumerate() {
        let mut prv = i + 1;

        // 内部のやつ
        ans += c * (c - 1) / 2;

        for m in 1.. {
            let idx = rle.lower_bound(&(a * (m + 1), 0));

            // debug!(rle[i], prv, idx, S[idx] - S[prv]);

            ans += c * m * (S[idx] - S[prv]);
            prv = idx;

            if idx == rle.len() {
                break;
            }
        }
    }

    println!("{ans}");
}

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

mod run_length {
    #![allow(dead_code)]
    //! ランレングス圧縮
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
}
