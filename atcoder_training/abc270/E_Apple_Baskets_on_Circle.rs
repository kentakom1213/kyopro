//       E - Apple Baskets on Circle       
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc270/tasks/abc270_e
// ----------------------------------------

/*

## 方針
m周したときに食べるりんごの数はk個以下？？
↓
二分探索

 */

use std::str::FromStr;
use std::fmt::Debug;

fn read_ints<T>() -> Vec<T>
where T: FromStr, <T as FromStr>::Err : Debug {
    let mut s =  String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|c| T::from_str(c).unwrap()).collect()
}

macro_rules! get_vals {
    ($($t:ty), *) => {
        {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            let mut iter = s.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
        }
    };
}

// solve
fn main() {
    let (n, mut k) = get_vals!(usize, isize);
    let mut a: Vec<isize> = read_ints();

    let is_ok = |m: isize| -> bool {
        let mut s: isize = 0;
        for i in 0..n {
            s += m.min(a[i]);
        }
        return s <= k;
    };

    // 二分探索
    let (mut ok, mut ng) = (0, 1001001001001);
    while ng - ok > 1 {
        let m = (ok + ng) / 2;
        if is_ok(m) {
            ok = m;
        } else {
            ng = m;
        }
    }

    // ok個のりんごを食べる
    for i in 0..n {
        let d = ok.min(a[i]);
        a[i] -= d;
        k -= d;
    }

    // 残りのりんごを食べる
    for i in 0..n {
        if k <= 0 {
            break;
        }
        if a[i] > 0 {
            a[i] -= 1;
            k -= 1;
        }
    }

    for &a_i in &a {
        print!("{} ", a_i);
    }
    println!();
}