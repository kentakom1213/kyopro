#![allow(non_snake_case)]

use itertools::Itertools;
use memoise::memoise_map;
use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        L: usize,
        A: [usize; N],
        B: [usize; M],
        C: [usize; L],
    }

    // 座標圧縮
    let comp = A
        .iter()
        .chain(&B)
        .chain(&C)
        .sorted()
        .dedup()
        .cloned()
        .collect_vec();

    let a = enc(&A, &comp);
    let b = enc(&B, &comp);
    let c = enc(&C, &comp);

    // 高橋が勝つか
    let takahashi_win = dfs(a, b, c);

    println!("{}", if takahashi_win { "Takahashi" } else { "Aoki" });
}

/// 先手番が勝つか
#[memoise_map(my, others, table)]
fn dfs(my: [u8; 13], others: [u8; 13], table: [u8; 13]) -> bool {
    for x in 0..13 {
        if my[x] == 0 {
            continue;
        }

        {
            // 場に出すだけ
            let mut new_my = my;
            new_my[x] -= 1;
            let mut new_table = table;
            new_table[x] += 1;

            if !dfs(others, new_my, new_table) {
                return true;
            }
        }

        for y in 0..x {
            if table[y] == 0 {
                continue;
            }

            {
                // 交換する
                let mut new_my = my;
                new_my[x] -= 1;
                new_my[y] += 1;
                let mut new_table = table;
                new_table[x] += 1;
                new_table[y] -= 1;

                if !dfs(others, new_my, new_table) {
                    return true;
                }
            }
        }
    }

    false
}

/// ビット列にエンコード
fn enc(S: &Vec<usize>, comp: &Vec<usize>) -> [u8; 13] {
    let mut res = [0; 13];
    for v in S {
        // A上でのvのインデックス
        let i = comp.binary_search(v).unwrap();
        res[i] += 1;
    }
    res
}
