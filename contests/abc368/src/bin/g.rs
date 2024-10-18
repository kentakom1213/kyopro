#![allow(non_snake_case)]

use cp_library_rs::{
    algebraic_structure::operation::Add, data_structure::segment_tree::SegmentTree, debug,
};
use proconio::{fastout, input, source::Readable};

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        mut B: [usize; N],
        Q: usize,
        queries: [Query; Q]
    }

    let mut segA = SegmentTree::<Add<usize>>::from(A);
    let mut over2_B = B
        .iter()
        .map(|&x| (x >= 2) as isize)
        .collect::<SegmentTree<Add<isize>>>();

    for q in queries {
        debug!(q);

        match q {
            Query::UpdateA(i, x) => segA.update(i, x),
            Query::UpdateB(i, x) => {
                B[i] = x;
                if x >= 2 {
                    over2_B.update(i, 1);
                } else {
                    over2_B.update(i, 0);
                }
            }
            Query::GetRange(mut l, mut r) => {
                r -= 1;

                let mut v = segA[l];

                while l < r {
                    // B[i] が次に 2 以上になる位置を探す
                    let next_l = r.min(over2_B.max_right(l + 1, |x| x < 1).1);

                    debug!(l, next_l);

                    v += segA.get_range(l + 1..next_l);
                    v = (v + segA[next_l]).max(v * B[next_l]);

                    l = next_l;
                }

                println!("{v}");
            }
        }

        debug!(segA, over2_B);
    }
}

#[derive(Debug)]
enum Query {
    UpdateA(usize, usize),
    UpdateB(usize, usize),
    GetRange(usize, usize),
}

impl Readable for Query {
    type Output = Query;
    fn read<R: std::io::BufRead, S: proconio::source::Source<R>>(source: &mut S) -> Self::Output {
        let t = u8::read(source);
        let x = usize::read(source) - 1;
        let y = usize::read(source);
        match t {
            1 => Self::UpdateA(x, y),
            2 => Self::UpdateB(x, y),
            3 => Self::GetRange(x, y),
            _ => unreachable!(),
        }
    }
}
