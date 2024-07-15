use proconio::input;

fn main() {
    input! {
        N: usize,
        S: String,
    }

    let ans = S.replace("gachi", "yurufuwa");

    println!("{ans}");
}
