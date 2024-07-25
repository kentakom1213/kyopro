#![allow(non_snake_case)]

use crate::cp_library_rs::{chmin, graph::bellman_ford::bellman_ford, utils::consts::IINF};
use ::cp_library_rs::debug;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        M: usize,
        mut edges: [(Usize1, Usize1, isize); M]
    }

    // 重みを反転（最短路に帰着）
    edges.iter_mut().for_each(|e| e.2 = -e.2);

    // 逆辺
    let redges = edges.iter().map(|&(u, v, w)| (v, u, w)).collect_vec();

    // 頂点0からの最短路
    let (_, dist0, _) = bellman_ford(N, 0, &edges);

    debug!(dist0);

    // 頂点N-1からの最短路
    let (_, distN, _) = bellman_ford(N, N - 1, &redges);

    let mut ans = IINF;

    for i in 0..N {
        // 頂点iを経由するとき
        chmin! {
            ans,
            dist0[i] + distN[i]
        };
    }

    if ans <= -IINF {
        println!("inf");
    } else {
        println!("{}", -ans);
    }
}

// ==================== cp-library-rs ====================
mod cp_library_rs {
    #![allow(dead_code)]
    pub mod graph {
        pub mod bellman_ford {
            //! ベルマン・フォード法
            use crate::cp_library_rs::utils::consts::{IINF, INF};
            /// ベルマン・フォード法
            /// - 重み付きグラフの単一始点最短路を求める
            /// - 重みが負の場合にも対応
            /// - 各頂点`v`の距離`dist[v]`について
            ///   - vに到達不可能な場合     → [`IINF`]
            ///   - vへの最短路が求まる場合 → `(vへの最短路長)`
            ///   - vへの最短路がいくらでも小さくできる場合
            ///                            → `-IINF`
            ///
            /// 引数
            /// - `N` : 頂点数
            /// - `start` : 始点
            /// - `edges` : 有向辺 `(in, out, weight)` の集合
            ///
            /// 戻り値
            /// - `has_negative` : (グラフ全体で)負閉路を含むか
            /// - `dist` : 各頂点への最短路
            /// - `prev` : 各頂点の最短路について，その頂点の直前に経由した頂点
            ///
            /// 計算量
            /// - `$O(NM)$`
            pub fn bellman_ford(
                N: usize,
                start: usize,
                edges: &Vec<(usize, usize, isize)>,
            ) -> (bool, Vec<isize>, Vec<usize>) {
                let mut dist = vec![2 * IINF; N];
                dist[start] = 0;
                let mut prev = vec![INF; N];
                for _ in 0..N {
                    for &(u, v, w) in edges {
                        // 緩和
                        if dist[v] > dist[u] + w {
                            dist[v] = dist[u] + w;
                            prev[v] = u;
                        }
                    }
                }
                // 負閉路検出
                let mut has_negative = false;
                for &(u, v, w) in edges {
                    if dist[v] > dist[u] + w {
                        has_negative |= true;
                        // 始点から到達できる場合
                        if dist[u] < IINF {
                            dist[v] = -IINF;
                        }
                    }
                }
                // 到達できない頂点をINFに
                for u in 0..N {
                    if dist[u] > IINF {
                        dist[u] = IINF;
                    }
                }
                // 影響範囲を調べる（もう一度ベルマンフォード）
                for _ in 0..N {
                    for &(u, v, w) in edges {
                        if dist[u] == IINF || dist[v] == IINF {
                            continue;
                        }
                        // 緩和
                        if dist[v] > dist[u] + w {
                            dist[v] = -IINF;
                        }
                    }
                }
                (has_negative, dist, prev)
            }
        }
    }
    pub mod utils {
        pub mod consts {
            //! 定数
            /// MOD用の定数：`$998244353$`
            pub const MOD998: usize = 998244353;
            /// MOD用の定数：`$10^9 + 7$`
            pub const MOD107: usize = 1000000007;
            /// 十分大きい数（usize）
            pub const INF: usize = 1001001001001001001;
            /// 十分大きい数（isize）
            pub const IINF: isize = 1001001001001001001;
            /// usizeにおける`-1`の値
            pub const NEG1: usize = 1_usize.wrapping_neg();
            /// 英小文字（文字列）
            pub const ASCII_LOWERCASE_STR: &str = "abcdefghijklmnopqrstuvwxyz";
            /// 英小文字（配列)
            pub const ASCII_LOWERCASE_ARR: [char; 26] = [
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
            ];
            /// 英大文字（文字列）
            pub const ASCII_UPPERCASE_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
            /// 英大文字（配列）
            pub const ASCII_UPPERCASE_ARR: [char; 26] = [
                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
                'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
            ];
            /// 16進数の文字（小文字）
            pub const HEX_LOWER: [char; 16] = [
                '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
            ];
            /// 16進数の文字（大文字）
            pub const HEX_UPPER: [char; 16] = [
                '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
            ];
        }
    }
    pub mod chmin {
        /// `chmin!{x1, x2, ..., xn}`:`x1`,`x2`,...,`xn`のうち最小のものを、`x1`に代入する
        /// - 代入があったとき、`true`を返す
        #[macro_export]
        macro_rules! chmin {
            ( $a:expr, $b:expr $(,)* ) => {{
                if $a > $b {
                    $a = $b;
                    true
                } else {
                    false
                }
            }};
            ( $a:expr, $b:expr, $c:expr $(,$other:expr)* $(,)* ) => {{
                chmin! {
                    $a,
                    ($b).min($c)
                    $(,$other)*
                }
            }};
        }
    }
}
