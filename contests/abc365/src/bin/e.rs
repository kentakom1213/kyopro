#![allow(non_snake_case)]

use cp_library_rs::debug;
use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    let mut S = vec![0; N + 1];
    // S[0] = A[0];
    for i in 0..N {
        S[i + 1] = S[i] ^ A[i];
    }

    debug!(S);

    let mut ans = 0_usize;

    // bitごとに平面走査
    for b in 0..32 {
        let mut res = 0;
        let mut sum0 = 0;
        let mut sum1 = 0;
    
        for i in 0..N {
            if S[i + 1] >> b & 1 == 0 {
                sum0 += 1;
            } else {
                sum1 += 1;
            }

            if S[i] >> b & 1 == 0 {
                res += sum1;
            } else {
                res += sum0;
            }
        }

        ans += res << b;
    }

    println!("{ans}");
}
