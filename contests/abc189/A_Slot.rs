//                 A - Slot                
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc189/tasks/abc189_a
// ----------------------------------------

use std::str::FromStr;
use std::fmt::Debug;

fn read_ints<T>() -> Vec<T>
where T: FromStr, <T as FromStr>::Err : Debug {
    let mut s =  String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|c| T::from_str(c).unwrap()).collect()
}

fn read_chars() -> Vec<char> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().chars().filter(|&c| !c.is_whitespace()).collect()
}

fn main() {
    let res = read_chars();
    let first = res[0];
    let mut is_ok = true;
    for &c in &res {
        if c != first {
            is_ok = false;
        }
    }

    if is_ok {
        println!("Won");
    } else {
        println!("Lost");
    }
}
