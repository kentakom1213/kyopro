fn main() {
    let (n, k) = (5, 3);

    let combs = combinations(0, n, k);

    println!("Combinations of choose 3 from [0, 5).");
    for comb in &combs {
        println!("{:?}", comb);
    }

    // from slice
    let arr: Vec<char> = "abcde".chars().collect();

    println!("Combinations of choose 3 from `arr`.");
    for comb in &combinations_from_slice(&arr, 3) {
        println!("{:?}", comb);
    }
}

/// ## combinations
/// 組合せの全列挙
fn combinations(start: usize, end: usize, k: usize) -> Vec<Vec<usize>> {
    // 再帰の終了条件
    if k == 1 {
        return (start..end).map(|n| vec![n]).collect();
    }

    let mut res = vec![];
    for i in start..end {
        let rem = combinations(i + 1, end, k - 1);
        for mut rem_comb in rem {
            let mut comb = vec![i];
            comb.append(&mut rem_comb);
            res.push(comb);
        }
    }
    res
}

/// ## combinations_from_iter
/// イテレータから組合せを生成する
fn combinations_from_slice<T: Copy>(arr: &[T], k: usize) -> Vec<Vec<T>> {
    if k == 1 {
        return arr.iter().map(|&v| vec![v]).collect();
    }
    let mut res = vec![];
    for (i, &val) in arr.iter().enumerate() {
        let rem = combinations_from_slice(&arr[i + 1..], k - 1);
        for mut rem_comb in rem {
            let mut comb = vec![val];
            comb.append(&mut rem_comb);
            res.push(comb);
        }
    }
    res
}
