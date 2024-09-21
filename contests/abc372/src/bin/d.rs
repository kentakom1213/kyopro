#![allow(non_snake_case)]

use cp_library_rs::{debug, utils::iterutil::IterUtil};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        H: [Usize1; N]
    }

    let mut ans = vec![0; N];
    let mut cnt = vec![H[N - 1]];

    for i in (0..N - 1).rev() {
        ans[i] = cnt.len();

        // 自分より小さいものを0に
        while cnt.last().is_some_and(|&x| x < H[i]) {
            cnt.pop();
        }
        cnt.push(H[i]);
    }

    debug!(ans);

    println!("{}", ans.iter().join(" "));
}
