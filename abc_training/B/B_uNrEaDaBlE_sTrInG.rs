//          B - uNrEaDaBlE sTrInG          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc192/tasks/abc192_b
// ----------------------------------------

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("failed");
    let s = s.trim();

    if is_unreadable(&s) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn is_unreadable(s: &str) -> bool {
    let mut is_even_lower = true;
    let mut is_odd_upper = true;

    for (i, c) in s.chars().enumerate() {
        if i % 2 == 0 {
            is_even_lower &= c.is_lowercase();
        } else {
            is_odd_upper &= c.is_uppercase();
        }
    }

    is_even_lower && is_odd_upper
}
