use cp_library_rs::debug;
use fixedbitset::FixedBitSet;
use proconio::input;

#[allow(non_snake_case)]

fn main() {
    input! {
        N: usize,
        M: usize,
        A: [[usize; M]; N]
    }

    let mut bt = vec![FixedBitSet::with_capacity(MAX_N); N];
    let mut bs = vec![FixedBitSet::with_capacity(MAX_N); 1000];

    for i in 0..M {
        for j in 0..N {
            bs[A[j][i]].set(j, true);
        }
        for j in 0..N {
            bt[j] ^= &bs[A[j][i]];
        }
        for j in 0..N {
            bs[A[j][i]].set(j, false);
        }
    }

    let mut ans = 0;

    for i in 0..N {
        ans += bt[i].count_ones(..);
    }

    debug!(ans);

    if M & 1 == 1 {
        ans -= N;
    }

    println!("{}", ans / 2);
}

const MAX_N: usize = 2000;
