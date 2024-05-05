//               E - Notebook              
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc273/tasks/abc273_e
// ----------------------------------------


#![allow(non_snake_case)]

use proconio::input;
use std::collections::HashMap;

#[proconio::fastout]
fn main() {
    input!(Q: usize);

    let mut tree = vec![ (-1, 0) ];
    let mut cur: usize = 0;
    let mut map = HashMap::new();

    for _ in 0..Q {
        input!(q: String);

        match &q[0..1] {
            "A" => {
                input!(x: isize);

                tree.push( (x, cur) );
                cur = tree.len() - 1;
            },
            "D" => {
                cur = tree[cur].1;
            },
            "S" => {
                input!(x: isize);
                map.insert(x, cur);
            },
            "L" => {
                input!(x: isize);
                cur = match map.get(&x) {
                    Some(i) => *i,
                    None => 0,
                };
            },
            _ => unreachable!(),
        }
        print!("{} ", tree[cur].0);
    }
    println!();
}
