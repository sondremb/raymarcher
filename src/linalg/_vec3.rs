use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign, Rem};
use crate::linalg::Vec2;
use crate::linalg::Vec4;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {x, y, z}
    }

    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalized(&self) -> Self {
        (*self / self.len())
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        vec3!(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x
        )
    }

    pub fn abs(&self) -> Self {
        vec3!(
            self.x.abs(),
            self.y.abs(),
            self.z.abs()
        )
    }

    pub fn min(&self, num: f64) -> Self {
        vec3!(
            self.x.min(num),
            self.y.min(num),
            self.z.min(num)
        )
    }

    pub fn max(&self, num: f64) -> Self {
        vec3!(
            self.x.max(num),
            self.y.max(num),
            self.z.max(num)
        )
    }

    pub fn clamp(&self, lower: f64, upper: f64) -> Self {
        self.max(lower).min(upper)
    }

    pub fn modulo(&self, modulus: f64) -> Self {
        ((*self % modulus) + modulus) % modulus
    }
}

impl_vec_ops!(Vec3; [x, y, z] (Add, add, AddAssign, add_assign => +));
impl_vec_ops!(Vec3; [x, y, z] (Sub, sub, SubAssign, sub_assign => -));
impl_vec_ops!(Vec3; [x, y, z] (Mul, mul, MulAssign, mul_assign => *));
impl_vec_ops!(Vec3; [x, y, z] (Div, div, DivAssign, div_assign => /));

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        vec3!(-self.x, -self.y, -self.z)
    }
}

impl Rem<f64> for Vec3 {
    type Output = Self;
    fn rem(self, rhs: f64) -> Self {
        vec3!(
            self.x % rhs,
            self.y % rhs,
            self.z % rhs
        )
    }
}

// This is but a drop in the ocean compared to Vec4
// You have been warned
swizzle!((Vec3) xx, x, x);
swizzle!((Vec3) xy, x, y);
swizzle!((Vec3) xz, x, z);
swizzle!((Vec3) yx, y, x);
swizzle!((Vec3) yy, y, y);
swizzle!((Vec3) yz, y, z);
swizzle!((Vec3) zx, z, x);
swizzle!((Vec3) zy, z, y);
swizzle!((Vec3) zz, z, z);
swizzle!((Vec3) xxx, x, x, x);
swizzle!((Vec3) xxy, x, x, y);
swizzle!((Vec3) xxz, x, x, z);
swizzle!((Vec3) xyx, x, y, x);
swizzle!((Vec3) xyy, x, y, y);
swizzle!((Vec3) xyz, x, y, z);
swizzle!((Vec3) xzx, x, z, x);
swizzle!((Vec3) xzy, x, z, y);
swizzle!((Vec3) xzz, x, z, z);
swizzle!((Vec3) yxx, y, x, x);
swizzle!((Vec3) yxy, y, x, y);
swizzle!((Vec3) yxz, y, x, z);
swizzle!((Vec3) yyx, y, y, x);
swizzle!((Vec3) yyy, y, y, y);
swizzle!((Vec3) yyz, y, y, z);
swizzle!((Vec3) yzx, y, z, x);
swizzle!((Vec3) yzy, y, z, y);
swizzle!((Vec3) yzz, y, z, z);
swizzle!((Vec3) zxx, z, x, x);
swizzle!((Vec3) zxy, z, x, y);
swizzle!((Vec3) zxz, z, x, z);
swizzle!((Vec3) zyx, z, y, x);
swizzle!((Vec3) zyy, z, y, y);
swizzle!((Vec3) zyz, z, y, z);
swizzle!((Vec3) zzx, z, z, x);
swizzle!((Vec3) zzy, z, z, y);
swizzle!((Vec3) zzz, z, z, z);
swizzle!((Vec3) xxxx, x, x, x, x);
swizzle!((Vec3) xxxy, x, x, x, y);
swizzle!((Vec3) xxxz, x, x, x, z);
swizzle!((Vec3) xxyx, x, x, y, x);
swizzle!((Vec3) xxyy, x, x, y, y);
swizzle!((Vec3) xxyz, x, x, y, z);
swizzle!((Vec3) xxzx, x, x, z, x);
swizzle!((Vec3) xxzy, x, x, z, y);
swizzle!((Vec3) xxzz, x, x, z, z);
swizzle!((Vec3) xyxx, x, y, x, x);
swizzle!((Vec3) xyxy, x, y, x, y);
swizzle!((Vec3) xyxz, x, y, x, z);
swizzle!((Vec3) xyyx, x, y, y, x);
swizzle!((Vec3) xyyy, x, y, y, y);
swizzle!((Vec3) xyyz, x, y, y, z);
swizzle!((Vec3) xyzx, x, y, z, x);
swizzle!((Vec3) xyzy, x, y, z, y);
swizzle!((Vec3) xyzz, x, y, z, z);
swizzle!((Vec3) xzxx, x, z, x, x);
swizzle!((Vec3) xzxy, x, z, x, y);
swizzle!((Vec3) xzxz, x, z, x, z);
swizzle!((Vec3) xzyx, x, z, y, x);
swizzle!((Vec3) xzyy, x, z, y, y);
swizzle!((Vec3) xzyz, x, z, y, z);
swizzle!((Vec3) xzzx, x, z, z, x);
swizzle!((Vec3) xzzy, x, z, z, y);
swizzle!((Vec3) xzzz, x, z, z, z);
swizzle!((Vec3) yxxx, y, x, x, x);
swizzle!((Vec3) yxxy, y, x, x, y);
swizzle!((Vec3) yxxz, y, x, x, z);
swizzle!((Vec3) yxyx, y, x, y, x);
swizzle!((Vec3) yxyy, y, x, y, y);
swizzle!((Vec3) yxyz, y, x, y, z);
swizzle!((Vec3) yxzx, y, x, z, x);
swizzle!((Vec3) yxzy, y, x, z, y);
swizzle!((Vec3) yxzz, y, x, z, z);
swizzle!((Vec3) yyxx, y, y, x, x);
swizzle!((Vec3) yyxy, y, y, x, y);
swizzle!((Vec3) yyxz, y, y, x, z);
swizzle!((Vec3) yyyx, y, y, y, x);
swizzle!((Vec3) yyyy, y, y, y, y);
swizzle!((Vec3) yyyz, y, y, y, z);
swizzle!((Vec3) yyzx, y, y, z, x);
swizzle!((Vec3) yyzy, y, y, z, y);
swizzle!((Vec3) yyzz, y, y, z, z);
swizzle!((Vec3) yzxx, y, z, x, x);
swizzle!((Vec3) yzxy, y, z, x, y);
swizzle!((Vec3) yzxz, y, z, x, z);
swizzle!((Vec3) yzyx, y, z, y, x);
swizzle!((Vec3) yzyy, y, z, y, y);
swizzle!((Vec3) yzyz, y, z, y, z);
swizzle!((Vec3) yzzx, y, z, z, x);
swizzle!((Vec3) yzzy, y, z, z, y);
swizzle!((Vec3) yzzz, y, z, z, z);
swizzle!((Vec3) zxxx, z, x, x, x);
swizzle!((Vec3) zxxy, z, x, x, y);
swizzle!((Vec3) zxxz, z, x, x, z);
swizzle!((Vec3) zxyx, z, x, y, x);
swizzle!((Vec3) zxyy, z, x, y, y);
swizzle!((Vec3) zxyz, z, x, y, z);
swizzle!((Vec3) zxzx, z, x, z, x);
swizzle!((Vec3) zxzy, z, x, z, y);
swizzle!((Vec3) zxzz, z, x, z, z);
swizzle!((Vec3) zyxx, z, y, x, x);
swizzle!((Vec3) zyxy, z, y, x, y);
swizzle!((Vec3) zyxz, z, y, x, z);
swizzle!((Vec3) zyyx, z, y, y, x);
swizzle!((Vec3) zyyy, z, y, y, y);
swizzle!((Vec3) zyyz, z, y, y, z);
swizzle!((Vec3) zyzx, z, y, z, x);
swizzle!((Vec3) zyzy, z, y, z, y);
swizzle!((Vec3) zyzz, z, y, z, z);
swizzle!((Vec3) zzxx, z, z, x, x);
swizzle!((Vec3) zzxy, z, z, x, y);
swizzle!((Vec3) zzxz, z, z, x, z);
swizzle!((Vec3) zzyx, z, z, y, x);
swizzle!((Vec3) zzyy, z, z, y, y);
swizzle!((Vec3) zzyz, z, z, y, z);
swizzle!((Vec3) zzzx, z, z, z, x);
swizzle!((Vec3) zzzy, z, z, z, y);
swizzle!((Vec3) zzzz, z, z, z, z);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_with_new() {
        let vec = Vec3 {
            x: 42.0,
            y: -69.0,
            z: 666.6
        };
        let vec_from_new = Vec3::new(42.0, -69.0, 666.6);
        assert_eq!(vec, vec_from_new);
    }

    #[test]
    fn create_from_macro_one() {
        let vec = Vec3::new(3.0, 3.0, 3.0);
        let vec_from_macro = vec3!(3);
        assert_eq!(vec, vec_from_macro);
    }

    #[test]
    fn create_from_macro_three() {
        let vec = Vec3::new(-1.0, 0.0, 1.0);
        let vec_from_macro = vec3!(-1, 0, 1);
        assert_eq!(vec, vec_from_macro);
    }

    #[test]
    fn len() {
        let vec = vec3!(4, 13, 16);
        assert_eq!(vec.len(), 21.0);
    }

    #[test]
    fn normalize() {
        let vec = vec3!(1, 2, 1);
        assert_eq!(vec.normalized().len(), 1.0);
    }

    #[test]
    fn dot() {
        let vec1 = vec3!(1, 2, 3);
        let vec2 = vec3!(-5, -2, 3);
        assert_eq!(vec1.dot(&vec2), 0.0);
        assert_eq!(vec1.dot(&vec1).sqrt(), vec1.len());
    }

    #[test]
    fn cross() {
        let vec1 = vec3!(5, 2, -4);
        let vec2 = vec3!(-2.5, -1, 2);
        assert_eq!(vec1.cross(&vec2), vec3!(0));
        let vec1 = vec3!(1, 0, 0);
        let vec2 = vec3!(0, 1, 0);
        assert_eq!(vec1.cross(&vec2), vec3!(0, 0, 1));
    }

    generate_op_test!(add_vec; vec3!(1), +, vec3!(2) => vec3!(3));
    generate_op_test!(add_float; vec3!(1), +, 0.2 => vec3!(1.2));
    generate_op_test!((assign) add_assign_vecs; vec3!(1), +=, vec3!(2) => vec3!(3));
    generate_op_test!((assign) add_assign_float; vec3!(1), +=, 68.0 => vec3!(69));

    generate_op_test!(sub_vec; vec3!(42), -, vec3!(12) => vec3!(30));
    generate_op_test!(sub_float; vec3!(39), -, 18.0 => vec3!(21));
    generate_op_test!((assign) sub_assign_vec; vec3!(44), -=, vec3!(26) => vec3!(18));
    generate_op_test!((assign) sub_assign_float; vec3!(97), -=, 82.0 => vec3!(15));

    generate_op_test!(mul_vec; vec3!(8), *, vec3!(4) => vec3!(32));
    generate_op_test!(mul_float; vec3!(10), *, 2.5 => vec3!(25));
    generate_op_test!((assign) mul_assign_vec; vec3!(44), *=, vec3!(0.25) => vec3!(11));
    generate_op_test!((assign) mul_assign_float; vec3!(13), *=, 3.0 => vec3!(39));

    generate_op_test!(div_vec; vec3!(42), /, vec3!(21) => vec3!(2));
    generate_op_test!(div_float; vec3!(39), /, 13.0 => vec3!(3));
    generate_op_test!((assign) div_assign_vec; vec3!(44), /=, vec3!(4) => vec3!(11));
    generate_op_test!((assign) div_assign_float; vec3!(99), /=, 3.0 => vec3!(33));

    #[test]
    fn neg() {
        let vec = vec3!(1, -2, 3);
        assert_eq!(-vec, vec3!(-1, 2, -3));
    }

    #[test]
    fn swizzle_assign() {
        let vec = vec3!(1.0, 2.0, 3.0);
        assert_eq!(vec3!(v2 vec.yx(), 5), vec3!(2, 1, 5));
    }
}