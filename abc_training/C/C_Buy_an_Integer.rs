//            C - Buy an Integer           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc146/tasks/abc146_c
// ----------------------------------------

#![allow(non_snake_case)]
// [Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)
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
            get!($t)
        ).collect::<Vec<_>>()
    };
    ($($t:ty),* ; $n:expr) => {
        (0..$n).map(|_|
            get!($($t),*)
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
            get!($t ;;)
        ).collect::<Vec<_>>()
    };
}

/// d(x) := xの10進表示での桁数
/// `N<=X`という条件のもと、`A*N + B*d(N)`を最大化する
fn main() {
    let (A, B, X) = get!(usize, usize, usize);
    
    let isOK = |n: usize| -> bool {
        A * n + B * n.to_string().len() <= X
    };
    
    let (mut l, mut r) = (0, 1_000_000_001);
    while (r - l) > 1 {
        let m = (l + r) / 2;
        if isOK(m) {
            l = m;
        } else {
            r = m;
        }
    }
    
    println!("{}", l);
}
