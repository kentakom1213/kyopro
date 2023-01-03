#![allow(dead_code)]

/// ## RunLengthEncode
/// ランレングス圧縮
fn run_length_encode<T>(arr: &[T]) -> Vec<(usize, T)>
where T: PartialEq + Copy
{
    let mut res = vec![];
    let mut cur = arr[0];
    let mut cnt = 1;
    for &val in &arr[1..] {
        if val == cur {
            cnt += 1;
        } else {
            res.push((cnt, cur));
            cur = val;
            cnt = 1;
        }
    }
    let last_elem = *arr.last().unwrap();
    res.push((cnt, last_elem));

    res
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_usize() {
        let arr = vec![0, 1, 1, 3, 3, 3, 2, 2, 1, 5, 9, 0];
        let comp = run_length_encode(&arr);
        let ans = vec![(1, 0), (2, 1), (3, 3), (2, 2), (1, 1), (1, 5), (1, 9), (1, 0)];

        assert_eq!(comp, ans);
    }

    #[test]
    fn test_string() {
        let strs = vec!["Welcome", "to", "Moo", "Moo", "Moo", "nsi", "nsi", "nsi", "nsi", "...", "nside."];
        let comp = run_length_encode(&strs);
        let ans = vec![(1, "Welcome"), (1, "to"), (3, "Moo"), (4, "nsi"), (1, "..."), (1, "nside.")];
        // [引用] "Mother2", nintendo, 1989

        assert_eq!(comp, ans);
    }

    #[test]
    fn test_chars() {
        let str = "aaaxbbbbbbccddef";
        let chars: Vec<char> = str.chars().collect();
        let comp = run_length_encode(&chars);
        let ans = vec![(3, 'a'), (1, 'x'), (6, 'b'), (2, 'c'), (2, 'd'), (1, 'e'), (1, 'f')];

        assert_eq!(comp, ans);
    }
}
