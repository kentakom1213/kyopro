#![allow(non_snake_case)]

use cp_library_rs::{debug, utils::yesno::YesNo};
use proconio::input;

/// Ci := Bi - Ai とおくと，
///
/// 操作: 整数 i,j を選び
/// - Ci -= 2
/// - Cj += 1
///
/// を繰り返して Ci = 0 にできるか？
///
/// という問題に言い換えられる．
///
/// ---
///
/// 観察
/// - Ci を正の数だけにできれば OK
/// - Ci < 0 の場合，マイナスを消すために Cj > 0 なる要素を Cj -= 2*Ci しないといけない
fn main() {
    input! {
        N: usize,
        A: [i64; N],
        B: [i64; N],
    }

    let C: Vec<_> = A.iter().zip(&B).map(|(&a, &b)| b - a).collect();

    debug!(C);

    let mut Sm = 0;
    let mut Sp = 0;

    for &c in &C {
        if c < 0 {
            Sm += c;
        } else {
            Sp += c / 2;
        }
    }

    debug!(Sm, Sp);

    let isok = Sm + Sp >= 0;

    println!("{}", isok.yesno());
}
