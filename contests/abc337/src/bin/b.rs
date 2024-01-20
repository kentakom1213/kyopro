#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use itertools::{EitherOrBoth, Itertools};
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

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        S: String
    }

    if S.len() == 0 {
        println!("Yes");
        return;
    }

    let rle = run_length_encode_from(S.chars());

    let iseq = |s: &str| {
        rle.iter().zip_longest(s.chars()).all(|s| match s {
            EitherOrBoth::Both(a, b) => a.0 == b,
            EitherOrBoth::Left(a) => false,
            EitherOrBoth::Right(b) => false,
        })
    };

    if iseq("ABC")
        || iseq("AB")
        || iseq("BC")
        || iseq("AC")
        || iseq("A")
        || iseq("B")
        || iseq("C")
        || iseq("")
    {
        println!("Yes");
    } else {
        println!("No");
    }
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
