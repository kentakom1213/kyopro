#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::{operation::Max, to_acted::ToActed},
    data_structure::implicit_treap::ImplicitTreap,
    tree::show_binary_tree::ShowBinaryTree,
};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        X: usize,
        Q: usize,
        AB: [(usize, usize); Q]
    }

    let mut set = ImplicitTreap::<ToActed<Max<usize>>>::default();
    set.push_back(X);

    for (i, &(a, b)) in AB.iter().enumerate() {
        let (_, aidx) = set.max_right(0, |x| x < a);
        set.insert(aidx, a);

        let (_, bidx) = set.max_right(0, |x| x < b);
        set.insert(bidx, b);

        set.print_as_binary_tree();

        let mid = set.get(i + 1);

        println!("{mid}");
    }
}
