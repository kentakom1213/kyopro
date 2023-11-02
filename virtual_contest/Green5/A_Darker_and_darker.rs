// https://atcoder.jp/contests/agc033/tasks/agc033_a

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
        h: usize,
        w: usize,
        a: [String; h],
    }

    let field: Vec<Vec<char>> = a.iter().map(|row| row.chars().collect()).collect();

    let mut dist = vec![vec![-1; w]; h];

    // BFS
    let mut deq = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if field[i][j] == '#' {
                dist[i][j] = 0;
                deq.push_back((i, j));
            }
        }
    }

    const MOVE: [(isize, isize); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

    while !deq.is_empty() {
        let (cr, cc) = deq.pop_front().unwrap();
        for &(dr, dc) in &MOVE {
            let (nr, nc) = (cr+dr, cc+dc);
            if 0 <= nr && nr < h
            && 0 <= nc && nc < w
            && dist[nr][nc] == -1 {
                dist[nr][nc] = dist[cr][cc] + 1;
                deq.push_back((nr, nc));
            }
        }
    }

    println!("{:?}", dist);
}
