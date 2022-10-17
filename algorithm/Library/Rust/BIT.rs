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
        let size = src.len() + 1;
        let mut arr = vec![0; size];
        for i in 1..size-1 {
            let x = src[i - 1];
            arr[i] += x;
            let j = i + i.wrapping_neg();
            if j < size {
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

    fn prefix_sum(&self, mut i: usize) -> isize {
        let mut res = 0;
        while i != 0 {
            res += self.arr[i];
            i -= i & i.wrapping_neg();
        }
        res
    }
}
