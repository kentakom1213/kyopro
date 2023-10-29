//          E - Come Back Quickly          
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc191/tasks/abc191_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

// imports
use std::collections::{HashMap, BTreeMap, VecDeque, BinaryHeap};
use std::cmp::Reverse;

// input macro
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
            ($(iter.next().unwrap().parse::<$t>().unwrap(),)*)
        }
    };
    ($t:ty ; $n:expr) => {
        (0..$n).map(|_| get!($t)).collect::<Vec<_>>()
    };
    ($($t:ty),* ; $n:expr) => {
        (0..$n).map(|_| get!($($t),*)).collect::<Vec<_>>()
    };
    ($t:ty ;;) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace().map(|t| t.parse::<$t>().unwrap()).collect::<Vec<_>>()
        }
    };
    ($t:ty ;; $n:expr) => {
        (0..$n).map(|_| get!($t ;;)).collect::<Vec<_>>()
    };
}

// static vales
static MOD1: usize = 1_000_000_007;
static MOD9: usize = 998_244_353;
static INF: usize = 1001001001001001001;


// solve
fn main() {
    let (N, M) = get!(usize, usize);
    // あらかじめ辺を圧縮する
    let mut edges: HashMap<(usize, usize), usize> = HashMap::new();
    for i in 0..M {
        let (mut a, mut b, c) = get!(usize, usize, usize);
        a -= 1; b -=1;
        edges.entry((a, b)).and_modify(|x| *x = c.min(*x)).or_insert(c);
    }

    let mut G1 = vec![vec![]; N];
    let mut G2 = vec![vec![]; N];
    for ((a, b), c) in &edges {
        G1[*a].push((*b, *c));
        G2[*b].push((*a, *c));
    }

    // 全頂点間ダイクストラ
    for i in 0..N {
        let mut res = INF;

        // 自己ループを検証
        if let Some(c) = edges.get(&(i, i)) {
            res = *c;
        }

        // ダイクストラ
        let go = dijkstra(&G1, i);
        let back = dijkstra(&G2, i);

        for j in 0..N {
            if i == j { continue; }
            res = res.min(go[j] + back[j]);
        }
        if res == INF {
            println!("-1");
        } else {
            println!("{}", res);
        }
    }
}

// Dijkstra法
fn dijkstra(graph: &Vec<Vec<(usize, usize)>>, s: usize) -> Vec<usize> {
    const INF: usize = 1001001001001001001;
    let n: usize = graph.len();
    let mut dist: Vec<usize> = vec![INF; n];
    let mut pq: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new();
    // 初期化
    dist[s] = 0;
    pq.push(Reverse((dist[s], s)));
    // 更新
    while let Some(Reverse((cost, u))) = pq.pop() {
        if dist[u] < cost { continue; }
        for &(v, weight) in &graph[u] {
            if dist[v] > dist[u] + weight {
                dist[v] = dist[u] + weight;
                pq.push(Reverse((dist[v], v)));
            }
        }
    }
    dist
}

