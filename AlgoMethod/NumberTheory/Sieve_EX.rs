use crate::get_ as get;

use self::segmented_sieve::segmented_sieve;

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

fn main() {
    // 入力受取り
    let (A, B) = get!(usize, usize);

    let primes = segmented_sieve(A, B + 1);

    println!("{}", primes.len());
}

mod segmented_sieve {
    //! 区間篩
    /// 区間 [l,r) の素数を列挙する
    pub fn segmented_sieve(l: usize, r: usize) -> Vec<usize> {
        let l = l.max(2);
        let r = r.max(l);
        let MAX = (r as f64).sqrt() as usize + 10;
        let mut divisors = vec![true; MAX];
        divisors[0] = false;
        divisors[1] = false;
        let mut sieve = vec![true; r - l];
        for p in 2..MAX {
            if !divisors[p] {
                continue;
            }
            let mut i = 2;
            while p * i < MAX {
                divisors[p * i] = false;
                i += 1;
            }
            let mut k = (l + p - 1) / p * p;
            while k < r {
                if k > p {
                    if l <= k && k < r {
                        sieve[k - l] = false;
                    }
                }
                k += p;
            }
        }
        sieve
            .iter()
            .zip(l..)
            .filter_map(|(&is_prime, x)| if is_prime { Some(x) } else { None })
            .collect()
    }
}

mod get_macro {
    //! 入力用マクロ
    // [Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)
    #[macro_export]
    macro_rules! get_ {
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
