#![allow(non_snake_case)]

use std::process::exit;

use memoise::memoise_map;
use num_traits::Pow;

fn main() {
    let (N, L, R) = get!(usize, usize, usize);

    let MAX = 1 << N;

    let ans = rec(L, R + 1, 0, MAX);

    println!("! {}", ans);
}

#[memoise_map(begin, end)]
fn rec(left: usize, right: usize, begin: usize, end: usize) -> usize {
    if end <= left || right <= begin {
        0
    } else if left <= begin && end <= right {
        // [begin, end)を聞く
        let i = {
            let mut pow = end - begin;
            let mut lg = 0_usize;
            while pow > 1 {
                lg += 1;
                pow >>= 1;
            }
            lg
        };
        let j = begin / (1 << i);

        assert_eq!(2.pow(i) as usize * j, begin);
        assert_eq!(2.pow(i) as usize * (j + 1), end);

        println!("? {} {}", i, j);
        let res = get!(isize);

        res as usize
    } else {
        let mid = (begin + end) / 2;
        let lval = rec(left, right, begin, mid) % 100;
        let rval = rec(left, right, mid, end) % 100;
        (lval + rval) % 100
    }
}

// fn get_inner(
//     &mut self,
//     left: usize,
//     right: usize,
//     begin: usize,
//     end: usize,
//     idx: usize,
// ) -> M::X {
//     // 遅延値を評価
//     self.eval(idx, end - begin);
//     // 区間を含まない
//     if end <= left || right <= begin {
//         M::IX
//     }
//     // 区間を包含する
//     else if left <= begin && end <= right {
//         self.data[idx].clone()
//     }
//     // 区間が重なる
//     else {
//         let mid = (begin + end) / 2;
//         let l_val = self.get_inner(left, right, begin, mid, idx * 2);
//         let r_val = self.get_inner(left, right, mid, end, idx * 2 + 1);
//         M::operate_x(&l_val, &r_val)
//     }
// }

mod macro_get {
    #![allow(dead_code)]
    //! 入力用マクロ
    //! - 参考：[Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)
    /// 入力用マクロ
    #[macro_export]
    macro_rules! get {
        ($t:ty) => {
            {
                let mut line = String::new();
                std::io::stdin().read_line(&mut line).unwrap();
                line.trim().parse::<$t>().unwrap()
            }
        };
        ($($t:ty),*) => {
            {
                let mut line = String::new();
                std::io::stdin().read_line(&mut line).unwrap();
                let mut iter = line.split_whitespace();
                (
                    $(iter.next().unwrap().parse::<$t>().unwrap(),)*
                )
            }
        };
        ($t:ty ; $n:expr) => {
            (0..$n).map(|_|
                get_!($t)
            ).collect::<Vec<_>>()
        };
        ($($t:ty),* ; $n:expr) => {
            (0..$n).map(|_|
                get_!($($t),*)
            ).collect::<Vec<_>>()
        };
        ($t:ty ;;) => {
            {
                let mut line = String::new();
                std::io::stdin().read_line(&mut line).unwrap();
                line.split_whitespace()
                    .map(|t| t.parse::<$t>().unwrap())
                    .collect::<Vec<_>>()
            }
        };
        ($t:ty ;; $n:expr) => {
            (0..$n).map(|_|
                get_!($t ;;)
            ).collect::<Vec<_>>()
        };
    }
}

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
