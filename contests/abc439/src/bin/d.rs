#![allow(non_snake_case)]

use cp_library_rs::debug;
use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        N: usize,
        mut A: [usize; N]
    }

    let mut ans = count(&A);

    A.reverse();

    ans += count(&A);

    println!("{ans}");
}

fn count(A: &[usize]) -> usize {
    let mut cnt = FxHashMap::default();
    let mut ans = 0;

    for &a in A {
        debug!(a, cnt);
        if a % 5 == 0 {
            let x = 7 * a / 5;
            let y = 3 * a / 5;
            if let Some((&cnt_x, &cnt_y)) = cnt.get(&x).zip(cnt.get(&y)) {
                ans += cnt_x * cnt_y;
            }
        }
        // a を追加
        *cnt.entry(a).or_insert(0) += 1_usize;
    }

    ans
}
