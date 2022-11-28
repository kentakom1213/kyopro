/// ソート済み配列において、`v`以上の最小のインデックスを取得
fn lower_bound(arr: &[usize], v: usize) -> usize {
    let mut ng = -1;
    let mut ok = arr.len() as isize;
    while (ok - ng) > 1 {
        let mid = ((ng + ok) as usize) / 2;
        if v <= arr[mid] {
            ok = mid as isize;
        } else {
            ng = mid as isize;
        }
    }
    ok as usize
}

/// ソート済み配列において、`v`より大きい最小のインデックスを取得
fn upper_bound(arr: &[usize], v: usize) -> usize {
    let mut ng = -1;
    let mut ok = arr.len() as isize;
    while (ok - ng) > 1 {
        let mid = ((ng + ok) as usize) / 2;
        if v < arr[mid] {
            ok = mid as isize;
        } else {
            ng = mid as isize;
        }
    }
    ok as usize
}
