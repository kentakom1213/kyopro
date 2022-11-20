// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// input macro
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
    // ($($t:ty);*) => {
    //     (
    //         $(get!($t),)*
    //     )
    // };
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


// static value
static V: usize = 4;
static E: usize = V * (V-1) / 2;
static ERR_CODE: usize = 5;

// SOLVE
struct Graph {
    block: usize,
    err: usize,
    v_size: usize,
    adj: Vec<Vec<bool>>,
}

impl Graph {
    /// 初期化
    fn new(block: usize, err: usize) -> Self {
        let v_size = block * err;
        let mut graph = Graph {
            block: block,
            err: err,
            v_size: v_size,
            adj: vec![vec![false; v_size]; v_size],
        };

        // ブロック内で結合させる
        for i in 0..block {
            for u in 0..err {
                for v in u+1..err {
                    let (x, y) = (i*err + u, i*err + v);
                    graph.adj[x][y] = true;
                    graph.adj[y][x] = true;
                }
            }
        }
        graph
    }

    /// 文字列から生成
    fn from_string(n: usize, edges: String, err: usize) -> Self {
        let mut graph = Graph::new(n, err);
        let mut iter = edges.chars();

        for u in 0..n {
            for v in u+1..n {
                let is_connected = iter.next().unwrap() == '1';
                if is_connected {
                    graph.adj[u][v] = true;
                }
            }
        }

        graph
    }

    /// 結合させる
    fn merge(&mut self, u: usize, v: usize) {
        let (u, v) = (u * self.err, v * self.err);
        for i in 0..self.err {
            for j in 0..self.err {
                let (x, y) = (u + i, v + j);
                self.adj[x][y] = true;
                self.adj[y][x] = true;
            }
        }
    }

    /// 結合判定
    fn is_connected(&self, u: usize, v: usize) -> bool {
        let mut count = 0;
        let (u, v) = (u * self.err, v * self.err);
        for i in 0..self.err {
            for j in 0..self.err {
                let (x, y) = (u + i, v + j);
                if self.adj[x][y] {
                    count += 1;
                }
            }
        }
        // 多数決で結果を出力
        count > self.err * self.err / 2
    }

    /// 文字列として出力
    fn to_string(&self) -> String {
        let mut res = String::new();
        for u in 0..self.v_size {
            for v in u+1..self.v_size {
                let c = if self.adj[u][v] { '1' } else { '0' };
                res.push(c);
            }
        }
        res
    }

    /// デバッグ用表示
    fn show(&self) {
        for i in 0..self.v_size {
            for j in 0..self.v_size {
                eprint!("{}", if self.adj[i][j] {1} else {0});
            }
            eprintln!();
        }
    }
}

/// 頂点数を与えられたとき、頂点を結ぶ辺のリストを返す
fn get_verb_list(n: usize) -> Vec<(usize, usize)> {
    let mut res = vec![];
    for i in 0..n {
        for j in i+1..n {
            res.push((i, j));
        }
    }
    res
}

// main
fn main() {
    let (M, eps) = get!(usize, f64);

    let v_all = V * ERR_CODE;

    // 頂点の組のリスト
    let v_list = get_verb_list(V);

    // グラフの出力
    println!("{}", v_all);
    for m in 0..M {
        let mut graph = Graph::new(V, ERR_CODE);
        for i in 0..E {
            if m >> i & 1 == 1 {
                let (x, y) = v_list[i];  // ブロック
                graph.merge(x, y);
            }
        }
        println!("{}", graph.to_string());
    }

    // クエリの処理
    for _ in 0..100 {
        let H = get!(String);
        let graph = Graph::from_string(v_all, H, ERR_CODE);
        
        // 多数決で決める
        let mut cnt = 0;
        for (i, &(u, v)) in v_list.iter().enumerate() {
            if graph.is_connected(u, v) {
                cnt += 1 << i;
            }
        }

        let res = (M-1).min(cnt);
        println!("{}", res)
    }
}
