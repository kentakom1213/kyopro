#![allow(non_snake_case)]

use cp_library_rs::{debug, trie::Trie};
use proconio::input;

fn main() {
    input! {
        S: String
    }

    let mut trie = Trie::new();

    for i in 0..S.len() {
        for j in i + 1..=S.len() {
            trie.insert(&S[i..j], ());
        }
    }

    debug!(trie.traverse());

    println!("{}", trie.len());
}
