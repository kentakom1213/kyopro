#![allow(non_snake_case)]

use cp_library_rs::{chmin, debug, utils::consts::INF};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        Q: usize,
        HT: [(char, Usize1); Q]
    }

    // dp[i] := 動かせる方の手の位置が i のときの最小コスト
    let mut dp = vec![INF; N];
    dp[1] = 0;

    debug!(dp);

    let mut ph = 0;
    let mut pt = 0;

    for &(h, t) in &HT {
        let h = (h == 'R') as usize;

        let mut ndp = vec![INF; N];

        for j in 0..N {
            if dp[j] == INF {
                continue;
            }

            // [左手の位置, 右手の位置]
            let mut pos = [0, 0];
            pos[ph] = pt;
            pos[ph ^ 1] = j;

            debug!(h, t, j, pos);

            for (cost, npos) in move_hand(pos[h], t, pos[h ^ 1], N) {
                debug!(cost, npos);
                chmin!(ndp[npos], dp[j] + cost);
            }
        }

        ph = h;
        pt = t;

        dp = ndp;
        debug!(dp);
    }

    let ans = dp.iter().min().unwrap();

    println!("{ans}");
}

/// 手を移動させる際のコストを計算する
///
/// **引数**
/// - `s`: 移動する手の初期位置
/// - `t`: 移動する手の目的位置
/// - `x`: 移動しない手の初期位置
/// - `N`: 周の長さ
///
/// **戻り値**
/// - [(移動コスト, 移動しない手の現在位置)]
fn move_hand(mut s: usize, mut t: usize, mut x: usize, N: usize) -> [(usize, usize); 2] {
    s %= N;
    t %= N;
    x %= N;

    let inc = |a: usize| if a == N - 1 { 0 } else { a + 1 };
    let dec = |a: usize| if a == 0 { N - 1 } else { a - 1 };

    let sub = |a: usize, b: usize| (a + N - b) % N;

    // 右回転で a → b に移動するコスト
    let rotR = |a: usize, b: usize| (b + N - a) % N;
    // 左回転で a → b に移動するコスト
    let rotL = |a: usize, b: usize| (a + N - b) % N;

    if s == t {
        return [(0, x), (0, x)];
    }

    // 重なっている場合
    if t == x {
        return [
            (rotR(s, t) + rotR(x, inc(x)), inc(x)),
            (rotL(s, t) + rotL(x, dec(x)), dec(x)),
        ];
    }

    // 重なっていない場合
    let (s_, t_) = (sub(s, x), sub(t, x));

    if s_ <= t_ {
        [(rotR(s, t), x), (rotL(s, t) + rotL(x, dec(t)), dec(t))]
    } else {
        [(rotL(s, t), x), (rotR(s, t) + rotR(x, inc(t)), inc(t))]
    }
}
