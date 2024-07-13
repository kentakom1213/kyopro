#![allow(non_snake_case)]

use crate::cp_library_rs::debug;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        LR: [(isize, isize); N]
    }

    let (mini, maxi) = LR.iter().fold((0, 0), |mut x, &(l, r)| {
        x.0 += l;
        x.1 += r;
        x
    });

    if maxi < 0 || 0 < mini {
        println!("No");
        return;
    }

    // 貪欲に増やす
    let (mut ans, _): (Vec<isize>, Vec<isize>) = LR.iter().cloned().unzip();

    let mut sum = mini;

    for i in 0..N {
        let (l, r) = LR[i];
        let ok = (r - l).min(-sum).max(0);

        sum += ok;
        ans[i] += ok;
    }

    assert_eq!(sum, 0);
    assert!((0..N).all(|i| LR[i].0 <= ans[i] && ans[i] <= LR[i].1));

    println!("Yes");
    println!("{}", ans.iter().join(" "));
}

// ==================== cp-library-rs ====================
mod cp_library_rs {
    #![allow(dead_code)]
    pub mod debug {
        /// デバッグ用マクロ
        #[macro_export]
        macro_rules! debug {
            ( $($val:expr),* $(,)* ) => {{
                #[cfg(debug_assertions)]
                eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
            }};
        }
    }
}
