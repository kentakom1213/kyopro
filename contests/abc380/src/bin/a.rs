#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: String
    }

    let one = N.chars().filter(|&c| c == '1').count();
    let two = N.chars().filter(|&c| c == '2').count();
    let three = N.chars().filter(|&c| c == '3').count();

    if [one, two, three] == [1, 2, 3] {
        println!("Yes");
    } else {
        println!("No");
    }
}
