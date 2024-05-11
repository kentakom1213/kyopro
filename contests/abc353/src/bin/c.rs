use itertools::Itertools;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    let mut acc = vec![0; N + 1];

    for i in 0..N {
        acc[i + 1] = acc[i] + A[i];
    }

    let mut S = 0;

    for i in 0..N {
        S += A[i] * (N - i - 1) + acc[N] - acc[i + 1];
    }

    debug!(S);

    // 超えた回数分だけ引く
    let sorted = A.iter().sorted().cloned().collect_vec();
    debug!(sorted);

    let mut cnt = 0;

    for i in 0..N {
        let idx = sorted.lower_bound(&(MOD - A[i]));
        if idx == N {
            continue;
        }
        let mut over = N - idx;
        if sorted[idx] <= A[i] {
            over -= 1;
        }
        debug!(i, A[i], idx, over);
        cnt += over;
    }

    debug!(cnt);

    S -= MOD * cnt / 2;

    println!("{S}");
}

const MOD: usize = 100000000;

mod macro_debug {
    #![allow(dead_code)]
    //! デバッグ用マクロ
    /// デバッグ用マクロ
    #[macro_export]
    macro_rules! debug {
        ( $($val:expr),* $(,)* ) => {{
            #[cfg(debug_assertions)]
            eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
        }};
    }
    /// 配列用マクロ
    #[macro_export]
    macro_rules! debug2D {
        ( $array:expr ) => {{
            #![cfg(debug_assertions)]
            eprintln!("{}: ", stringify!($array));
            for row in &$array {
                eprintln!("{:?}", row);
            }
        }};
    }
}
