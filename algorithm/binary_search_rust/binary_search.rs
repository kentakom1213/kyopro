fn main() {
    let vec = vec![1, 5, 5, 10, 11, 11, 15];

    let is_ok = |n: isize, m: isize| -> bool {
        n <= vec[m as usize]
    };

    // 条件を満たす最大を求める
    let lower_bound = |n: isize| {
        let mut l = -1_isize;
        let mut r = vec.len() as isize;
        while (r - l) > 1 {
            let mid = (l + r) / 2;
            if is_ok(n, mid) {
                r = mid;
            } else {
                l = mid;
            }
        }
        r
    };

    // 条件を満たさない最小を求める
    let upper_bound = |n: isize| {
        let mut l = -1_isize;
        let mut r = vec.len() as isize;
        while (r - l) > 1 {
            let mid = (l + r) / 2;
            if is_ok(n, mid) {
                l = mid;
            } else {
                r = mid;
            }
        }
        l
    };

    eprintln!("{:?}", vec);

    // lower_bound
    eprintln!("lb(-1) -> {}", lower_bound(-1));
    eprintln!("lb(1) -> {}", lower_bound(1));
    eprintln!("lb(3) -> {}", lower_bound(3));
    eprintln!("lb(5) -> {}", lower_bound(5));
    eprintln!("lb(7) -> {}", lower_bound(7));
    eprintln!("lb(11) -> {}", lower_bound(11));
    eprintln!("lb(100) -> {}", lower_bound(100));

    // upper_bound
    eprintln!("ub(-1) -> {}", upper_bound(-1));
    eprintln!("ub(1) -> {}", upper_bound(1));
    eprintln!("ub(3) -> {}", upper_bound(3));
    eprintln!("ub(5) -> {}", upper_bound(5));
    eprintln!("ub(7) -> {}", upper_bound(7));
    eprintln!("ub(11) -> {}", upper_bound(11));
    eprintln!("ub(100) -> {}", upper_bound(100));
}
