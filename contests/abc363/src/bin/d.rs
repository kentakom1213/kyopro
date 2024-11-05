#![allow(non_snake_case)]

use cp_library_rs::debug;
use proconio::input;

fn main() {
    input! {
        N: usize
    }

    if N == 1 {
        println!("0");
        return;
    }

    // 適切な桁数を求める
    let mut cnt = 1;
    let mut d = 0;

    for i in 1.. {
        let tmp = 9 * 10_usize.pow((i - 1) as u32 / 2);
        debug!(i, cnt, tmp);
        if cnt + tmp >= N {
            d = i;
            break;
        }

        cnt += tmp;
    }

    // 残り
    let rem = N - cnt;

    debug!(d, cnt, rem);

    let start = 10_usize.pow((d - 1) as u32 / 2);

    let left = rem + start - 1;

    let l = left.to_string();
    let r = l.chars().rev().collect::<String>();

    let ans = if d % 2 == 0 { l + &r } else { l + &r[1..] };

    println!("{ans}");
}
