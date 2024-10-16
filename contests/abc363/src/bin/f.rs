/*
 * 10^12 以下の最大の高度合成数：
 *
 * 9316358251200 = 2^6 ∗ 3^3 ∗ 5^2 ∗ 7 ∗ 11 ∗ 13 ∗ 17 ∗ 19 ∗ 23 ∗ 29
 *
 * - 約数の個数：10752個 → 約数の個数^2 くらいまではok
 *
 * - 参考：https://algo-method.com/descriptions/92
 */

#![allow(non_snake_case)]

use cp_library_rs::utils::iterutil::IterUtil;
use proconio::input;

fn main() {
    input! {N: usize}

    let res = find_palindrome(N);

    if let Some((left, right)) = res {
        let mut l = left.iter().rev().join("*");
        let r = right.iter().join("*");

        if !r.is_empty() {
            l += "*";
            l += &r;
        }

        println!("{l}");
    } else {
        println!("-1");
    }
}

fn find_palindrome(n: usize) -> Option<(Vec<usize>, Vec<usize>)> {
    if is_palindrome(n) && !n.to_string().contains('0') {
        return Some((vec![n], vec![]));
    }

    // 約数を全探索
    for i in 2.. {
        if i * i > n {
            break;
        }
        if n % i != 0 || i.to_string().contains('0') {
            continue;
        }
        let res = n / i;
        let j = rev_number(i);
        if res % j != 0 {
            continue;
        }
        if let Some((mut left, mut right)) = find_palindrome(res / j) {
            left.push(i);
            right.push(j);
            return Some((left, right));
        }
    }

    None
}

fn rev_number(n: usize) -> usize {
    n.to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

fn is_palindrome(n: usize) -> bool {
    let s = n.to_string();
    s.chars()
        .zip(s.chars().rev())
        .take(s.len() / 2)
        .all(|(x, y)| x == y)
}
