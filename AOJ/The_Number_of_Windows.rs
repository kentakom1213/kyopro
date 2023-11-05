//          The Number of Windows          
// ----------------------------------------
// 問題
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_3_C
// ----------------------------------------

use std::collections::VecDeque;

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

fn main() {
    let (N, Q) = get!(usize, usize);
    let A = get!(usize;;);
    let queries = get!(usize;;);

    for &X in &queries {
        // 尺取り法
        let mut sum = 0;
        let mut deq = VecDeque::new();
        let mut ans = 0;

        for i in 0..N {
            // 要素を追加
            deq.push_back(A[i]);
            sum += A[i];

            if sum <= X {
                ans += deq.len();
            }

            // 条件を満たすまで削除
            while !deq.is_empty() && sum > X {
                let top = deq.pop_front().unwrap();
                sum -= top;
                if sum <= X {
                    ans += deq.len();
                }
            }
        }

        println!("{}", ans);
    }
}
