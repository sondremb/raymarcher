use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign, Rem};
use crate::linalg::Vec3;
use crate::linalg::Vec4;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self {x, y}
    }

    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalized(&self) -> Self {
        (*self / self.len())
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y
    }


    pub fn abs(&self) -> Self {
        vec2!(
            self.x.abs(),
            self.y.abs()
        )
    }

    pub fn min(&self, num: f64) -> Self {
        vec2!(
            self.x.min(num),
            self.y.min(num)
        )
    }

    pub fn max(&self, num: f64) -> Self {
        vec2!(
            self.x.max(num),
            self.y.max(num)
        )
    }

    pub fn clamp(&self, lower: f64, upper: f64) -> Self {
        self.max(lower).min(upper)
    }

    pub fn modulo(&self, modulus: f64) -> Self {
        ((*self % modulus) + modulus) % modulus
    }
}

impl_vec_ops!(Vec2; [x, y] (Add, add, AddAssign, add_assign => +));
impl_vec_ops!(Vec2; [x, y] (Sub, sub, SubAssign, sub_assign => -));
impl_vec_ops!(Vec2; [x, y] (Mul, mul, MulAssign, mul_assign => *));
impl_vec_ops!(Vec2; [x, y] (Div, div, DivAssign, div_assign => /));

impl Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self {
        vec2!(-self.x, -self.y)
    }
}

impl Rem<f64> for Vec2 {
    type Output = Self;
    fn rem(self, rhs: f64) -> Self {
        vec2!(
            self.x % rhs,
            self.y % rhs
        )
    }
}

// Does this look like a lot of repeated lines?
// You should see Vec3...
swizzle!((Vec2) xx, x, x);
swizzle!((Vec2) xy, x, y);
swizzle!((Vec2) yx, y, x);
swizzle!((Vec2) yy, y, y);
swizzle!((Vec2) xxx, x, x, x);
swizzle!((Vec2) xxy, x, x, y);
swizzle!((Vec2) xyx, x, y, x);
swizzle!((Vec2) xyy, x, y, y);
swizzle!((Vec2) yxx, y, x, x);
swizzle!((Vec2) yxy, y, x, y);
swizzle!((Vec2) yyx, y, y, x);
swizzle!((Vec2) yyy, y, y, y);
swizzle!((Vec2) xxxx, x, x, x, x);
swizzle!((Vec2) xxxy, x, x, x, y);
swizzle!((Vec2) xxyx, x, x, y, x);
swizzle!((Vec2) xxyy, x, x, y, y);
swizzle!((Vec2) xyxx, x, y, x, x);
swizzle!((Vec2) xyxy, x, y, x, y);
swizzle!((Vec2) xyyx, x, y, y, x);
swizzle!((Vec2) xyyy, x, y, y, y);
swizzle!((Vec2) yxxx, y, x, x, x);
swizzle!((Vec2) yxxy, y, x, x, y);
swizzle!((Vec2) yxyx, y, x, y, x);
swizzle!((Vec2) yxyy, y, x, y, y);
swizzle!((Vec2) yyxx, y, y, x, x);
swizzle!((Vec2) yyxy, y, y, x, y);
swizzle!((Vec2) yyyx, y, y, y, x);
swizzle!((Vec2) yyyy, y, y, y, y);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_with_new() {
        let vec = Vec2 {
            x: 42.0,
            y: -69.0
        };
        let vec_from_new = Vec2::new(42.0, -69.0);
        assert_eq!(vec, vec_from_new);
    }

    #[test]
    fn create_from_macro_one() {
        let vec = Vec2::new(3.0, 3.0);
        let vec_from_macro = vec2!(3);
        assert_eq!(vec, vec_from_macro);
    }

    #[test]
    fn create_from_macro_two() {
        let vec = Vec2::new(-1.0, 0.0);
        let vec_from_macro = vec2!(-1, 0);
        assert_eq!(vec, vec_from_macro);
    }

    #[test]
    fn len() {
        let vec = vec2!(3, 4);
        assert_eq!(vec.len(), 5.0);
    }

    #[test]
    fn normalize() {
        let vec = vec2!(1, 2);
        assert_eq!(vec.normalized().len(), 1.0);
    }

    #[test]
    fn dot() {
        let vec1 = vec2!(1, 2);
        let vec2 = vec2!(-5, 2.5);
        assert_eq!(vec1.dot(&vec2), 0.0);
        assert_eq!(vec1.dot(&vec1).sqrt(), vec1.len());
    }

    generate_op_test!(add_vec; vec2!(1), +, vec2!(2) => vec2!(3));
    generate_op_test!(add_float; vec2!(1), +, 0.2 => vec2!(1.2));
    generate_op_test!((assign) add_assign_vecs; vec2!(1), +=, vec2!(2) => vec2!(3));
    generate_op_test!((assign) add_assign_float; vec2!(1), +=, 68.0 => vec2!(69));

    generate_op_test!(sub_vec; vec2!(42), -, vec2!(12) => vec2!(30));
    generate_op_test!(sub_float; vec2!(39), -, 18.0 => vec2!(21));
    generate_op_test!((assign) sub_assign_vec; vec2!(44), -=, vec2!(26) => vec2!(18));
    generate_op_test!((assign) sub_assign_float; vec2!(97), -=, 82.0 => vec2!(15));

    generate_op_test!(mul_vec; vec2!(8), *, vec2!(4) => vec2!(32));
    generate_op_test!(mul_float; vec2!(10), *, 2.5 => vec2!(25));
    generate_op_test!((assign) mul_assign_vec; vec2!(44), *=, vec2!(0.25) => vec2!(11));
    generate_op_test!((assign) mul_assign_float; vec2!(13), *=, 3.0 => vec2!(39));

    generate_op_test!(div_vec; vec2!(42), /, vec2!(21) => vec2!(2));
    generate_op_test!(div_float; vec2!(39), /, 13.0 => vec2!(3));
    generate_op_test!((assign) div_assign_vec; vec2!(44), /=, vec2!(4) => vec2!(11));
    generate_op_test!((assign) div_assign_float; vec2!(99), /=, 3.0 => vec2!(33));

    #[test]
    fn neg() {
        let vec = vec2!(1, -2);
        assert_eq!(-vec, vec2!(-1, 2));
    }
}