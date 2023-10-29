//                A - 幅優先探索
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/atc002/tasks/abc007_3
// ----------------------------------------

use proconio::{
    input,
    marker::{Chars, Isize1},
};
use std::collections::VecDeque;

const INF: isize = 1_000_000_000;
const MOVE: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

pub trait At {
    type Output;
    fn at(&self, index: isize) -> &Self::Output;
    fn at_mut(&mut self, index: isize) -> &mut Self::Output;
}

impl<T> At for Vec<T> {
    type Output = T;
    fn at(&self, index: isize) -> &Self::Output {
        self.get(index as usize).unwrap()
    }

    fn at_mut(&mut self, index: isize) -> &mut Self::Output {
        self.get_mut(index as usize).unwrap()
    }
}

fn main() {
    input! {
        R: usize,
        C: usize,
        sy: Isize1,
        sx: Isize1,
        gy: Isize1,
        gx: Isize1,
        S: [Chars; R],
    }

    let mut dist = vec![vec![INF; C]; R];
    *dist.at_mut(sy).at_mut(sx) = 0;
    let mut q = VecDeque::new();
    q.push_back((sy, sx));

    while let Some((cy, cx)) = q.pop_front() {
        for &(dy, dx) in &MOVE {
            let (ny, nx) = (cy + dy, cx + dx);
            if *S.at(ny).at(nx) == '#' || *dist.at(ny).at(nx) != INF {
                continue;
            }
            *dist.at_mut(ny).at_mut(nx) = *dist.at(cy).at(cx) + 1;
            q.push_back((ny, nx));
        }
    }

    println!("{}", *dist.at(gy).at(gx));
}
