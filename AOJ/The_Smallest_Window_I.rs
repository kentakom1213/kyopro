//          The Smallest Window I          
// ----------------------------------------
// 問題
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_3_A&lang=ja
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

const INF: usize = 1001001001001001001;

fn main() {
    let (n, s) = get!(usize, usize);
    let mut a = get!(usize;;);

    // 番兵
    a.push(0);

    // 尺取り法
    let mut sum = 0;
    let mut deq = VecDeque::new();
    let mut ans = INF;

    for i in 0..=n {
        // 条件を満たすギリギリまで削除
        while !deq.is_empty() && sum >= s {
            let x = deq.pop_front().unwrap();
            if sum - x < s {
                // 戻す
                deq.push_front(x);
                break;
            }
            sum -= x;
        }

        if sum >= s {
            ans = ans.min(deq.len());
        }

        // 追加
        deq.push_back(a[i]);
        sum += a[i];
    }

    if ans < INF {
        println!("{}", ans);
    } else {
        println!("0");
    }
}
