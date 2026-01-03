#![allow(non_snake_case)]

use ac_library::maxflow;
use proconio::input;

const INF: usize = usize::MAX;

fn main() {
    input! {
        N: usize,
        S: [String; N],
        A: [usize; N]
    }

    let mut flow = maxflow::MfGraph::<usize>::new(2 * N + 2);
    let source = 2 * N;
    let sink = 2 * N + 1;

    for i in 0..N {
        flow.add_edge(source, i, A[i]);
        flow.add_edge(i + N, sink, A[i]);
    }

    // 包含関係で辺を張る
    for i in 0..N {
        for j in 0..N {
            if i == j {
                continue;
            }
            if S[i] == S[j] {
                if i < j {
                    flow.add_edge(i, j + N, INF);
                }
            } else if S[j].contains(&S[i]) {
                flow.add_edge(i, j + N, INF);
            }
        }
    }

    let mf = flow.flow(source, sink);
    let sum = A.iter().sum::<usize>();

    println!("{}", sum - mf);
}
