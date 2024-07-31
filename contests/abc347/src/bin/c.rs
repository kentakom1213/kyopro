#![allow(non_snake_case)]

use cp_library_rs::{debug, utils::yesno::YesNo};
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        A: usize,
        B: usize,
        D: [usize; N]
    }

    let mut day = vec![];

    for &d in &D {
        let x = d % (A + B);
        day.push(x);
        day.push(x + A + B);
    }

    day.sort();
    day.dedup();

    debug!(day);

    // 間がB日以上あるか
    let isok = day.iter().tuple_windows::<(_, _)>().any(|(x, y)| y - x > B);

    println!("{}", isok.yesno());
}
