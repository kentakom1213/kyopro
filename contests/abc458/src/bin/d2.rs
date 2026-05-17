#![allow(non_snake_case)]

use cp_library_rs::{
    data_structure::indexedset::IndexedSet, debug, tree::show_binary_tree::ShowBinaryTree,
};
use proconio::input;

fn main() {
    input! {
        X: usize,
        Q: usize,
        AB: [(usize, usize); Q]
    }

    let mut set = IndexedSet::default();
    set.insert(X);

    for (i, &(a, b)) in AB.iter().enumerate() {
        set.insert(a);
        set.insert(b);

        set.print_as_binary_tree();
        debug!(set);

        let mid = set.nth(i + 1).unwrap();

        println!("{mid}");
    }
}
