//                 C - 背の順                 
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc041/tasks/abc041_c
// ----------------------------------------

use std::cmp::Reverse;
use std::str::FromStr;
use std::fmt::Debug;
use std::io::{stdout, StdoutLock, BufWriter, Write};

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

fn with_bufwriter<F: FnOnce(BufWriter<StdoutLock>) -> ()>(f: F) {
    let out = stdout();
    let writer = BufWriter::new(out.lock());
    f(writer)
}

fn main() {
    let n: u32 = read_ints()[0];
    let a: Vec<i32> = read_ints();

    let mut height_num: Vec<(i32, usize)> = a.iter().enumerate().map(|(i, &v)| (v, i+1)).collect();
    height_num.sort_by_key(|&x| Reverse(x));

    with_bufwriter(|mut out| {
        for (_, i) in height_num {
            writeln!(out, "{}", i).unwrap();
        }
    });
}

