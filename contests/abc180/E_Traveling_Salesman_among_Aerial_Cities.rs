// E - Traveling Salesman among Aerial Cities
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc180/tasks/abc180_e
// ----------------------------------------

/*

## 方針
- bitDP

### 定義
dp[i][S] := 現在地`i`、訪問済み都市の集合`S`のときのcostの最小値

### 更新
dp[i][S] = min{ dp[j][S\{i}] + dist(i,j) | j in S\{i} }

## 参考
- https://algo-logic.info/bit-dp/

 */

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

// solve
fn main() {
    let N = get!(usize);
    let xyz = get!(isize, isize, isize; N);

    // コストを計算
    let mut dist = vec![vec![0; N]; N];
    for i in 0..N {
        for j in 0..N {
            let (a, b, c) = xyz[i];
            let (p, q, r) = xyz[j];
            let cost = (p-a).abs() + (q-b).abs() + (r-c).max(0);
            dist[i][j] = cost;
        }
    }

    // bitDP
    let INF: isize = 1_000_000_000;
    let mut dp = vec![vec![INF; N]; 1 << N];
    dp[0][0] = 0;

    // 配る
    for S in 0..1<<N {
        // u->v に移動する
        for v in 0..N {
            for u in 0..N {
                if S != 0 && (S & (1 << u)) == 0 {
                    continue;  // Sがvを含まない場合
                }
                if (S & (1 << v)) == 0 {
                    if u != v {
                        let old_cost = dp[S | (1 << v)][v];
                        let new_cost = dp[S][u] + dist[u][v];
                        if old_cost > new_cost {
                            dp[S | (1 << v)][v] = new_cost;
                        }
                    }
                }
            }
        }
    }

    // for row in &dp {
    //     println!("{:?}", row);
    // }

    let ans = dp[(1<<N)-1].iter().enumerate().map(|(i,&c)| c+dist[i][0]).min().unwrap();
    println!("{}", ans);
}
