#![allow(dead_code)]

/// ## ord
/// `A`を`0`とするascii文字(`A~Za~z`)のインデックスを返す
fn ord(c: char) -> usize {
    let a = 'A' as u32;
    let c = c as u32;
    (c - a) as usize
}

/// ## chr
/// `chr(0) = A`であるようなascii文字(`A~Za~z`)を返す
fn chr(i: usize) -> char {
    let a = 'A' as u32;
    let c = char::from_u32(a + i as u32).unwrap();
    c
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ord() {
        for (i, c) in ('A'..'z').enumerate() {
            assert_eq!(ord(c), i);
        }
    }

    #[test]
    fn test_chr() {
        for (i, c) in ('A'..'z').enumerate() {
            assert_eq!(chr(i), c);
        }
    }
}
