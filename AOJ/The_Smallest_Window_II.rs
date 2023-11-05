//          The Smallest Window II
// ----------------------------------------
// 問題
// https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_3_B&lang=ja
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
    let (n, k) = get!(usize, usize);
    let a: Vec<usize> = get!(usize;;).into_iter().map(|x| x - 1).collect();

    // 尺取り法
    let mut count = vec![0; 101010];
    let mut rem = k;
    let mut deq = VecDeque::new();
    let mut ans = INF;

    for i in 0..n {
        // 追加
        deq.push_back(a[i]);
        if a[i] < k && count[a[i]] == 0 {
            rem -= 1;
        }
        count[a[i]] += 1;

        // 条件を満たさなくなるまで削除
        while rem == 0 && !deq.is_empty() {
            // 条件を満たしている場合判定
            if rem == 0 {
                // 判定
                if rem == 0 {
                    ans = ans.min(deq.len());
                }
            }
            // 削除
            let x = deq.pop_front().unwrap();
            count[x] -= 1;
            if x < k && count[x] == 0 {
                rem += 1;
            }
        }
    }

    if ans < INF {
        println!("{}", ans);
    } else {
        println!("0");
    }
}
