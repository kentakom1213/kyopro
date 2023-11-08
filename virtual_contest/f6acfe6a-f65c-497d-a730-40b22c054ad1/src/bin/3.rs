// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use im_rc::HashSet;
// imports
use itertools::Itertools;
use proconio::{input, marker::{Chars, Bytes, Usize1}};

macro_rules! debug {
    ( $($val:expr),* $(,)* ) => {{
        #[cfg(debug_assertions)]
        eprintln!( concat!($(stringify!($val), " = {:?}, "),*), $($val),* );
    }};
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;

fn main() {
    input! {
        N: usize,
        T: usize,
        M: usize,
        bad: [(Usize1, Usize1); M]
    }

    // 悪い組
    let mut badset = HashSet::new();
    for &(a, b) in &bad {
        badset.insert((a, b));
        badset.insert((b, a));
    }

    debug!(&badset);

    let mut ans = 0;
    make_team(&mut vec![vec![]; T], 0, 0, N, &badset, &mut ans);

    println!("{}", ans);
}

/// t: どのチームまで
/// a: どのメンバーまで
fn make_team(team: &mut Vec<Vec<usize>>, t: usize, a: usize, N: usize, bad: &HashSet<(usize, usize)>, ans: &mut usize) {
    // debug!(&team, t, a, ans);
    if a == N && t + 1 == team.len() {
        // debug!(&team, "OK");
        *ans += 1;
        return;
    } else if a >= N || t >= team.len() {
        return;
    }
    if t >= team.len() {
        return;
    }
    for i in 0..=t {
        // aを0~tに振り分ける
        if team[i].iter().all(|&b| !bad.contains(&(a, b))) {
            team[i].push(a);
            make_team(team, t, a + 1, N, bad, ans);
            team[i].pop();
        }
    }
    // iをt + 1(空)に振り分ける
    if t + 1 >= team.len() || team[t].is_empty() {
        return;
    }
    team[t + 1].push(a);
    make_team(team, t + 1, a + 1, N, bad, ans);
    team[t + 1].pop();
}
