#![allow(dead_code)]

use std::collections::BinaryHeap;
use std::cmp::Reverse;

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


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_dijkstra() {
        let G = vec![
            vec![(1, 2), (3, 4), (4, 5)],
            vec![(0, 2), (1, 6), (3, 3)],
            vec![(1, 6), (3, 2), (5, 4)],
            vec![(0, 4), (1, 3), (2, 2), (4, 2)],
            vec![(0, 5), (3, 2), (5, 6)],
            vec![(2, 4), (4, 6)],
        ];

        let dist = dijkstra(&G, 0);
        assert_eq!(dist, vec![0, 2, 6, 4, 5, 10]);
    }
}
