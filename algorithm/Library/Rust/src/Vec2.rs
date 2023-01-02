#![allow(dead_code)]

use std::ops::{Add, Sub, Mul};

/// Vector2D
/// - 二次元ベクトル
#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2<T> {
    x: T,
    y: T,
}

impl<T> Vec2<T>
where T: Copy
       + Add<Output = T>
       + Sub<Output = T>
       + Mul<Output = T>
{
    fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }

    fn mul(&self, scalar: T) -> Self {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    fn add(&self, other: Self) -> Self {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub(&self, other: Self) -> Self {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn dot(&self, other: Self) -> T {
        self.x * other.x + self.y * other.y
    }

    /// ## dist2
    /// 距離の2乗の値を返す
    fn dist2(&self, other: Self) -> T {
        (self.x - other.x) * (self.x - other.x)
        + (self.y - other.y) * (self.y - other.y)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_sub_mul_vec2() {
        let a = Vec2::new(2, 3);
        let b = Vec2::new(5, -2);

        assert_eq!(a.add(b), Vec2 { x:7, y:1 });
        assert_eq!(a.sub(b), Vec2 { x:-3, y:5 });

        let a2 = a.mul(-2);
        let b2 = b.mul(3);
        assert_eq!(a2.add(b2), Vec2 { x:11, y:-12 });
    }

    #[test]
    fn test_dot() {
        let a = Vec2::new(2.0, -5.0);
        let b = Vec2::new(10.0, 4.0);

        assert_eq!(a.dot(b), 0.0);
    }

    #[test]
    fn test_dist2() {
        let zero = Vec2::new(0, 0);
        let a = Vec2::new(1, 2);
        let b = Vec2::new(2, 1);

        let dist_0_a = a.dist2(zero);
        let dist_0_b = b.dist2(zero);
        assert_eq!(dist_0_a, dist_0_b);

        let dist_a_b = a.dist2(b);
        assert_eq!(dist_a_b, 2);
    }
}