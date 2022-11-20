// sub17

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// const
const INF: usize = 1001001001001001001;
const SHUFFLE_COUNT: usize = 100;
const SEED: [u8; 32] = [2, 0, 0, 2, 1, 2, 1, 3,
                        2, 0, 0, 4, 0, 9, 1, 0,
                        2, 0, 1, 0, 0, 8, 2, 1,
                        2, 0, 1, 3, 0, 3, 2, 2];
const SPLIT_RATE: usize = 5;  // `V / SPLIT_RATE`個の点を切り離す

// crates
use rand;
use rand::prelude::*;

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
    ($($t:ty);*) => {
        (
            $(get!($t),)*
        )
    };
}

// SOLVE
#[derive(Clone)]
struct Graph {
    v_size: usize,
    adj: Vec<Vec<bool>>,
    indegree: Vec<usize>,
}

impl Graph {
    /// 初期化
    fn new(v_size: usize) -> Self {
        Graph {
            v_size: v_size,
            adj: vec![vec![false; v_size]; v_size],
            indegree: vec![0; v_size],
        }
    }

    /// 文字列からグラフを生成
    fn from_string(n: usize, edges: String) -> Self {
        let mut graph = Graph::new(n);
        let mut iter = edges.chars();

        for u in 0..n {
            for v in u+1..n {
                let is_connected = iter.next().unwrap() == '1';
                if is_connected {
                    graph.adj[u][v] = true;
                    graph.adj[v][u] = true;
                    graph.indegree[u] += 1;
                    graph.indegree[v] += 1;
                }
            }
        }
        graph
    }

    /// 2頂点を結合し、入次数を更新する
    fn merge(&mut self, u: usize, v: usize) {
        if u == v {
            return;
        }
        if !self.adj[u][v] {
            self.adj[u][v] = true;
            self.indegree[u] += 1;
        }
        if !self.adj[v][u] {
            self.adj[v][u] = true;
            self.indegree[v] += 1;
        }
    }

    /// 2頂点を分割し、入次数を更新する
    fn split(&mut self, u: usize, v: usize) {
        if u == v {
            return;
        }
        if self.adj[u][v] {
            self.adj[u][v] = false;
            self.indegree[u] -= 1;
        }
        if self.adj[v][u] {
            self.adj[v][u] = false;
            self.indegree[v] -= 1;
        }
    }

    /// 頂点`u`を他全ての頂点から切り離す
    fn split_point(&mut self, u: usize) {
        for v in 0..self.v_size {
            self.split(u, v);
        }
    }

    /// 頂点`u`を他全ての頂点に接続する
    fn merge_point(&mut self, u: usize) {
        for v in 0..self.v_size {
            if u == v { continue; }
            self.merge(u, v);
        }
    }

    /// 2頂点の状態に応じて、結合、切断をトグルする
    fn toggle(&mut self, u: usize, v: usize) {
        if self.adj[u][v] {
            self.split(u, v);
        } else {
            self.merge(u, v);
        }
    }

    /// 与えられたスペクトルをもつグラフを生成する
    fn from_spectrum(v_size: usize, i: usize) -> Self {
        let mut graph = Graph::new(v_size);
        // 結合させる
        for j in 1..=i*i {
            for u in 0..v_size {
                let v = (u + j) % v_size;
                if graph.indegree[u] < i && graph.indegree[v] < i {
                    graph.merge(u, v);
                }
            }
        }
        graph
    }

    /// 与えられたスペクトル（2つ）をもつグラフを生成する
    fn from_spectrum2(v_size: usize, a: usize, b: usize) -> Self {
        let mut graph = Graph::new(v_size);
        let block = v_size / 2;

        // 結合させる
        for i in 1..=a*a {
            for u in 0..block {
                let v = (u + i) % block;
                if graph.indegree[u] < a && graph.indegree[v] < a {
                    graph.merge(u, v);
                }
            }
        }
        // 結合させる
        for i in 1..=b.pow(2) {
            for u in 0..block {
                let v = (u + i) % block;
                let (u, v) = (u + block, v + block);
                if graph.indegree[u] < b && graph.indegree[v] < b {
                    graph.merge(u, v);
                }
            }
        }
        graph
    }

    /// `Graph.deg_list()[i]` := 入次数`i`の頂点数
    fn deg_list(&self) -> Vec<usize> {
        let mut res = vec![0; self.v_size];
        for &i in &self.indegree {
            res[i] += 1;
        }
        res
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
        eprintln!();
    }
}

/// ベクトル同士の二乗和誤差
fn sq_error(vec_a: &Vec<usize>, vec_b: &Vec<usize>) -> usize {
    let mut res = 0;
    for (&a, &b) in vec_a.iter().zip(vec_b.iter()) {
        if a >= b {
            res += (a - b).pow(2);
        } else {
            res += (b - a).pow(2);
        }
    }
    res
}

// main
fn main() {
    // 乱数生成器を初期化
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::from_seed(SEED);

    // 入力受け取り
    let (M, eps) = get!(usize, f64);

    // グラフの設定
    let V = M;
    println!("{}", V);

    // ランダムに削除する頂点の数
    let SPLIT_VERB = V / SPLIT_RATE;

    // クラスタリング用のグラフ情報
    let mut graph_spectrum = vec![vec![0; V]; M];

    // グラフを構成する
    for i in 0..M {
        let mut graph = Graph::from_spectrum(M, i);

        // ランダムに選んだ点をグラフから切り離す
        if i % 3 == 1 {
            let mut verbs: Vec<usize> = (0..V).collect();
            verbs.shuffle(&mut rng);

            for &u in &verbs[..SPLIT_VERB] {
                graph.split_point(u);
            }
        }
        // ランダムに選んだ点をグラフから切り離す
        if i % 3 == 2 {
            let mut verbs: Vec<usize> = (0..V).collect();
            verbs.shuffle(&mut rng);

            for &u in &verbs[..SPLIT_VERB] {
                graph.merge_point(u);
            }
        }
        
        // 生成したグラフを出力
        println!("{}", graph.to_string());

        // `SHUFFLE_COUNT`回行う
        for _ in 0..SHUFFLE_COUNT {
            // クローンしたグラフ
            let mut g = graph.clone();

            // 辺の有無を確率`eps`でランダムに変化させる
            for u in 0..V {
                for v in u+1..V {
                    let rand_val = rng.gen::<f64>();
                    if rand_val <= eps {
                        g.toggle(u, v);
                    }
                }
            }
            
            for (j, v) in g.deg_list().iter().enumerate() {
                graph_spectrum[i][j] += v;
            }
        }
    }

    // クエリ処理
    for _ in 0..100 {
        let H_str = get!(String);
        let H = Graph::from_string(V, H_str);
        let degs = H.deg_list().iter().map(|&v| v * SHUFFLE_COUNT).collect();

        // 各パラメータに関して二乗和誤差を求め、最も小さいものを採用する
        let mut ans = (0, INF);  // idx, err
        for (i, v) in graph_spectrum.iter().enumerate() {
            let err = sq_error(v, &degs);

            if err < ans.1 {
                ans = (i, err);
            }
        }

        // 答えの出力
        println!("{}", ans.0);
    }
}
