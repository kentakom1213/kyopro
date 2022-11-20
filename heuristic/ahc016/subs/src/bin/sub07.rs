// sub07

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

static INF: usize = 1001001001001001001;

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

    /// 頂点を結合し、入次数を更新する
    fn merge(&mut self, u: usize, v: usize) {
        if !self.adj[u][v] {
            self.adj[u][v] = true;
            self.indegree[u] += 1;
        }
        if !self.adj[v][u] {
            self.adj[v][u] = true;
            self.indegree[v] += 1;
        }
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
    let (M, eps) = get!(usize, f64);

    // グラフの設定
    let V = M;
    println!("{}", V);

    // クラスタリング用のグラフ情報
    let mut graph_spectrum = vec![];

    // グラフを構成する
    for i in 0..M {
        let mut graph = Graph::new(V);

        // 結合させる
        for j in 1..=i*i {
            for u in 0..V {
                let v = (u + j) % V;
                if graph.indegree[u] < i && graph.indegree[v] < i {
                    graph.merge(u, v);
                }
            }
        }
        println!("{}", graph.to_string());
        graph_spectrum.push(graph.deg_list());
    }

    // クエリ処理
    for _ in 0..100 {
        let H_str = get!(String);
        let H = Graph::from_string(V, H_str);
        let degs = H.deg_list();
        
        eprintln!("{:?}", degs);
        
        // 各パラメータに関して二乗和誤差を求め、最も小さいものを採用する
        let mut ans = (0, INF);  // idx, err
        for (i, v) in graph_spectrum.iter().enumerate() {
            let err = sq_error(v, &degs);
            
            // eprintln!("i:{} err:{}", i, err);

            if err < ans.1 {
                ans = (i, err);
            }
        }

        // 答えの出力
        eprintln!("{}\n", ans.0);
        println!("{}", ans.0);
    }
}
