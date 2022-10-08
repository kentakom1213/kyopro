//            D - Root M Leaper            
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc272/tasks/abc272_d
// ----------------------------------------

// [Rustで競技プログラミングの入力をスッキリ記述するマクロ](https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8)
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

// solve
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    // i^2 + j^2 == m となる (i, j) を取得
    let mut moves = vec![];
    for i in 0..m {
        if i*i > m { break; };
        for j in 0..m {
            if j*j > m { break; };
            if i*i + j*j == m {
                moves.push((i as isize, j as isize));
            }
        }
    }

    let sign: Vec<(isize, isize)> = vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];

    // bfs
    let mut deq = VecDeque::new();
    deq.push_back((0, 0));

    const FILL: isize = 1_000_000_000_000_000;
    let mut field = vec![vec![FILL; n]; n];

    while !deq.is_empty() {
        let (cr, cc) = deq.pop_front().unwrap();
        
        for (dr, dc) in moves {
            for (sr, sc) in sign {
                let nr = cr + sr * dr;
                let nc = cc + sc * dc;
                if can_reach(nr, nc, n) && field[nr][nc] > field[cr][cc] + 1 {
                    field[nr][nc] = field[cr][cc] + 1;
                    deq.push((nr, nc));
                }
            }
        }
    }
}

/// 探索可能な範囲かを判定する
fn can_reach(r: isize, c: isize, n: usize) -> bool {
    let n = n as isize;
    0 <= r && r < n && 0 <= c && c < n
}
