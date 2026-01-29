#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::actedmonoid::ActedMonoid, data_structure::implicit_treap::ImplicitTreap,
    utils::show_binary_tree::ShowBinaryTree,
};
use itertools::Itertools;
use proconio::input;
use rustc_hash::FxHashMap;

fn main() {
    input! {
        N: usize,
        LR: [(usize, usize); N],
        Q: usize,
        X: [usize; Q]
    }

    let mut tree = ImplicitTreap::<AddMax>::default();

    for &x in X.iter().sorted() {
        tree.push_back((x, x));
    }

    tree.print_as_binary_tree();

    for &(l, r) in &LR {
        // l 未満のインデックス
        let ((x, _), lb) = tree.max_right(0, |v| v.0 < l);

        // r 以下のインデックス
        let (_, ub) = tree.max_right(0, |v| v.0 <= r);

        // 区間に 1 を足す
        tree.apply(lb..ub, 1);

        // 区間の切断
        let (left, right) = tree.split_nth(tree.root, ub);
        let (left, mid) = tree.split_nth(left, lb);

        let merged = tree.merge(left, right);

        // 挿入位置を特定
        tree.root = merged;
        let (_, idx) = tree.max_right(0, |v| v.0 < x + 1);

        // 挿入
        let (left, right) = tree.split_nth(tree.root, idx);

        let left = tree.merge(left, mid);
        let root = tree.merge(left, right);

        tree.root = root;

        tree.print_as_binary_tree();
    }

    let mut res = FxHashMap::default();

    for i in 0..Q {
        let (cur, init) = tree.get(i);
        res.insert(init, cur);
    }

    for &x in &X {
        println!("{}", res[&x]);
    }
}

/// 区間加算 + 区間最大値
#[derive(Debug)]
pub struct AddMax;
impl ActedMonoid for AddMax {
    type Val = (usize, usize);
    type Act = usize;
    fn e() -> Self::Val {
        (0, 0)
    }
    fn id() -> Self::Act {
        0
    }
    fn op(x: &Self::Val, y: &Self::Val) -> Self::Val {
        (x.0.max(y.0), x.1)
    }
    fn mapping(x: &Self::Val, y: &Self::Act) -> Self::Val {
        (x.0 + y, x.1)
    }
    fn compose(x: &Self::Act, y: &Self::Act) -> Self::Act {
        x + y
    }
}
