use cp_library_rs::debug;
use proconio::input;

fn main() {
    input! {
        A: isize,
        B: isize,
    }

    if A == B {
        println!("1");
        return;
    }

    let mut ans = 2;

    if (B - A) % 2 == 0 {
        debug!(2);
        ans += 1;
    }

    println!("{ans}");
}
