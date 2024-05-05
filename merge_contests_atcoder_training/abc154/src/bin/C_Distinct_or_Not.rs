//           C - Distinct or Not           
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc154/tasks/abc154_c
// ----------------------------------------

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let sorted = merge_sort(&a);
    let is_ok = sorted.iter().fold((true, -1), |acc, &x| (acc.0 && (acc.1 < x), x));
    if is_ok.0 {
        println!("YES");
    } else {
        println!("NO");
    }
}

/// マージソート
fn merge_sort(arr: &[i32]) -> Vec<i32> {
    let arr_size = arr.len();

    if arr_size == 1 {
        arr.to_vec()
    } else {
        let mid = arr_size / 2;
        let left = merge_sort( &arr[..mid] );  // 配列の左半分
        let right = merge_sort( &arr[mid..] );  // 配列の右半分

        // left, rightをマージ
        let mut sorted = Vec::<i32>::new();  // ソート済み配列
        let (l_size, r_size) = (left.len(), right.len());  // 左、右の配列サイズ
        let (mut l, mut r) = (0, 0);  // 左、右の配列を走査するindex
        while l < l_size || r < r_size {
            if (l < l_size && r < r_size && left[l] <= right[r]) || r == r_size {
                sorted.push(left[l]);
                l += 1;
            } else {
                sorted.push(right[r]);
                r += 1;
            }
        }
        sorted
    }
}

