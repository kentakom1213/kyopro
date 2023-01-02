#![allow(non_snake_case)]
#![allow(dead_code)]

// BinaryIndexedTree
struct BIT {
    size: usize,
    arr: Vec<isize>,
}

impl BIT {
    fn new(n: usize) -> Self {
        BIT {
            size: n,
            arr: vec![0; n+1],
        }
    }

    fn build(src: &[isize]) -> Self {
        let size = src.len();
        let mut arr = vec![0; size + 1];
        for i in 1..=size {
            let x = src[i - 1];
            arr[i] += x;
            let j = i + (i & i.wrapping_neg());
            if j < size + 1 {
                arr[j] += arr[i];
            }
        }
        Self {
            size,
            arr,
        }
    }

    fn add(&mut self, mut i: usize, x: isize) {
        i += 1;
        while i <= self.size {
            self.arr[i] += x;
            i += i & i.wrapping_neg();
        }
    }

    fn sum(&self, mut i: usize) -> isize {
        let mut res = 0;
        while i != 0 {
            res += self.arr[i];
            i -= i & i.wrapping_neg();
        }
        res
    }

    fn sum_range(&self, l: usize, r: usize) -> isize {
        let to_l = self.sum(l);
        let to_r = self.sum(r);
        to_r - to_l
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let mut bit = BIT::new(5);
        
        bit.add(0, 20);
        bit.add(2, -5);

        let sum_5 = bit.sum(5);
        assert_eq!(sum_5, 15);

        bit.add(4, 10);
        bit.add(1, -20);

        let sum_2 = bit.sum(2);
        assert_eq!(sum_2, 0);

        let sum_all = bit.sum(5);
        assert_eq!(sum_all, 5);
    }

    #[test]
    fn test_build() {
        let mut bit = BIT::build(&vec![1, 2, 3, 4, 5]);

        assert_eq!(bit.sum_range(1, 4), 9);
        assert_eq!(bit.sum(5), 15);

        bit.add(2, -3);
        bit.add(3, -4);

        assert_eq!(bit.sum(5), 8);
    }
}
