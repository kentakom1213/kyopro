#![allow(non_snake_case)]

use cp_library_rs::{debug, debug2D};
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        A: [Usize1; N]
    }

    // それぞれの個数をカウント
    let cnt = A.iter().fold([0; 4], |mut cnt, &x| {
        cnt[x] += 1;
        cnt
    });
    debug!(cnt);

    let mut S = [0; 5];
    for i in 0..4 {
        S[i + 1] = S[i] + cnt[i];
    }
    debug!(S);

    // map[x][y] := 本来xであるべき位置にyがあるような添字の個数
    let mut map = [[0; 4]; 4];

    let mut x = 0;
    for (i, &y) in A.iter().enumerate() {
        while S[x + 1] <= i {
            x += 1;
        }
        debug!(x, y);
        map[x][y] += 1;
    }

    debug2D!(map);

    // カウント
    let mut ans = 0;

    // 長さ1のサイクル
    for i in 0..4 {
        map[i][i] = 0;
    }
    debug2D!(map);

    // 長さ2のサイクル
    for perm in (0..4).permutations(2) {
        if let &[i, j] = &perm[..] {
            let x = map[i][j].min(map[j][i]);
            ans += x;
            map[i][j] -= x;
            map[j][i] -= x;
        }
    }
    debug2D!(map);

    // 長さ3のサイクル
    for perm in (0..4).permutations(3) {
        if let &[i, j, k] = &perm[..] {
            let x = map[i][j].min(map[j][k]).min(map[k][i]);
            ans += 2 * x;
            map[i][j] -= x;
            map[j][k] -= x;
            map[k][i] -= x;
        }
    }
    debug2D!(map);

    // 長さ4のサイクル
    for perm in (0..4).permutations(4) {
        if let &[i, j, k, l] = &perm[..] {
            let x = map[i][j].min(map[j][k]).min(map[k][l]).min(map[l][i]);
            ans += 3 * x;
            map[i][j] -= x;
            map[j][k] -= x;
            map[k][l] -= x;
            map[l][i] -= x;
        }
    }
    debug2D!(map);

    println!("{ans}");
}
