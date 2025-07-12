#![allow(non_snake_case)]

use proconio::input;
use rustc_hash::FxHashSet;

fn main() {
    input! {
        A: usize,
        N: usize
    }

    let Nstr = N.to_string();
    let Nhalf = Nstr[..Nstr.len() / 2 + 1].parse::<usize>().unwrap();

    // debug!(Nhalf);

    let mut ans = 0;

    for p in 1..=9 {
        if p > N {
            break;
        }
        if is_palin(p, A) {
            ans += p;
        }
    }

    for i in 1..=Nhalf {
        let istr = i.to_string();
        let irev = istr.chars().rev().collect::<String>();
        let pstr = istr + &irev;
        let p = pstr.parse::<usize>().unwrap();

        if p > N {
            break;
        }

        if is_palin(p, A) {
            ans += p;
        }
    }

    'outer: for i in 1..=Nhalf {
        for o in '0'..='9' {
            let istr = i.to_string();
            let irev = istr.chars().rev().collect::<String>();
            let pstr = istr + &o.to_string() + &irev;
            let p = pstr.parse::<usize>().unwrap();

            if p > N {
                break 'outer;
            }

            if is_palin(p, A) {
                ans += p;
            }
        }
    }

    println!("{ans}");
}

fn is_palin(n: usize, a: usize) -> bool {
    let mut ps = vec![];

    let mut x = n;
    while x > 0 {
        ps.push(x % a);
        x /= a;
    }

    let ps2 = ps.clone();
    ps.reverse();

    // debug!(n, ps, ps2);

    ps == ps2
}
