// https://atcoder.jp/contests/tenka1-2012-qualA/tasks/tenka1_2012_qualA_2

// solve
fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();

    let ans: Vec<&str> = s.split_whitespace().collect();
    println!("{}", ans.join(","));
}
