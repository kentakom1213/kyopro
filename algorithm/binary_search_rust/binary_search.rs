#![allow(non_snake_case)]

fn main() {
    let vec = vec![1, 5, 5, 10, 11, 11, 15];

    let isOK = |n: isize, m: isize| {
        n <= vec[m as usize]
    };

    // 条件を満たす最大を求める
    let lower_bound = |n: isize| {
        let mut l = -1_isize;
        let mut r = vec.len() as isize;
        while (r - l) > 1 {
            let mid = (l + r) / 2;
            if isOK(n, mid) {
                r = mid;
            } else {
                l = mid;
            }
        }
        l
    };

    // 条件を満たさない最小を求める
    let upper_bound = |n: isize| {
        let mut l = -1_isize;
        let mut r = vec.len() as isize;
        while (r - l) > 1 {
            let mid = (l + r) / 2;
            if n < vec[mid as usize] {
                r = mid;
            } else {
                l = mid;
            }
        }
        r
    };

    eprintln!("{:?}", vec);

    // lower_bound
    eprintln!("lower_bound(-1) -> {}", lower_bound(-1));
    eprintln!("lower_bound(1) -> {}", lower_bound(1));
    eprintln!("lower_bound(3) -> {}", lower_bound(3));
    eprintln!("lower_bound(5) -> {}", lower_bound(5));
    eprintln!("lower_bound(7) -> {}", lower_bound(7));
    eprintln!("lower_bound(11) -> {}", lower_bound(11));
    eprintln!("lower_bound(100) -> {}", lower_bound(100));

    // upper_bound
    eprintln!("upper_bound(-1) -> {}", upper_bound(-1));
    eprintln!("upper_bound(1) -> {}", upper_bound(1));
    eprintln!("upper_bound(3) -> {}", upper_bound(3));
    eprintln!("upper_bound(5) -> {}", upper_bound(5));
    eprintln!("upper_bound(7) -> {}", upper_bound(7));
    eprintln!("upper_bound(11) -> {}", upper_bound(11));
    eprintln!("upper_bound(100) -> {}", upper_bound(100));
}
