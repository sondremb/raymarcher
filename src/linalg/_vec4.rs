use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign, Rem};
use crate::linalg::Vec2;
use crate::linalg::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

impl Vec4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self {x, y, z, w}
    }

    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalized(&self) -> Self {
        (*self / self.len())
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn abs(&self) -> Self {
        vec4!(
            self.x.abs(),
            self.y.abs(),
            self.z.abs(),
            self.w.abs()
        )
    }

    pub fn min(&self, num: f64) -> Self {
        vec4!(
            self.x.min(num),
            self.y.min(num),
            self.z.min(num),
            self.w.min(num)
        )
    }

    pub fn max(&self, num: f64) -> Self {
        vec4!(
            self.x.max(num),
            self.y.max(num),
            self.z.max(num),
            self.w.max(num)
        )
    }

    pub fn clamp(&self, lower: f64, upper: f64) -> Self {
        self.max(lower).min(upper)
    }

    pub fn modulo(&self, modulus: f64) -> Self {
        ((*self % modulus) + modulus) % modulus
    }
}

impl_vec_ops!(Vec4; [x, y, z, w] (Add, add, AddAssign, add_assign => +));
impl_vec_ops!(Vec4; [x, y, z, w] (Sub, sub, SubAssign, sub_assign => -));
impl_vec_ops!(Vec4; [x, y, z, w] (Mul, mul, MulAssign, mul_assign => *));
impl_vec_ops!(Vec4; [x, y, z, w] (Div, div, DivAssign, div_assign => /));

impl Neg for Vec4 {
    type Output = Self;
    fn neg(self) -> Self {
        vec4!(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Rem<f64> for Vec4 {
    type Output = Self;
    fn rem(self, rhs: f64) -> Self {
        vec4!(
            self.x % rhs,
            self.y % rhs,
            self.z % rhs,
            self.w % rhs
        )
    }
}

// God has been dead for a very long time
swizzle!((Vec4) xx, x, x);
swizzle!((Vec4) xy, x, y);
swizzle!((Vec4) xz, x, z);
swizzle!((Vec4) xw, x, w);
swizzle!((Vec4) yx, y, x);
swizzle!((Vec4) yy, y, y);
swizzle!((Vec4) yz, y, z);
swizzle!((Vec4) yw, y, w);
swizzle!((Vec4) zx, z, x);
swizzle!((Vec4) zy, z, y);
swizzle!((Vec4) zz, z, z);
swizzle!((Vec4) zw, z, w);
swizzle!((Vec4) wx, w, x);
swizzle!((Vec4) wy, w, y);
swizzle!((Vec4) wz, w, z);
swizzle!((Vec4) ww, w, w);
swizzle!((Vec4) xxx, x, x, x);
swizzle!((Vec4) xxy, x, x, y);
swizzle!((Vec4) xxz, x, x, z);
swizzle!((Vec4) xxw, x, x, w);
swizzle!((Vec4) xyx, x, y, x);
swizzle!((Vec4) xyy, x, y, y);
swizzle!((Vec4) xyz, x, y, z);
swizzle!((Vec4) xyw, x, y, w);
swizzle!((Vec4) xzx, x, z, x);
swizzle!((Vec4) xzy, x, z, y);
swizzle!((Vec4) xzz, x, z, z);
swizzle!((Vec4) xzw, x, z, w);
swizzle!((Vec4) xwx, x, w, x);
swizzle!((Vec4) xwy, x, w, y);
swizzle!((Vec4) xwz, x, w, z);
swizzle!((Vec4) xww, x, w, w);
swizzle!((Vec4) yxx, y, x, x);
swizzle!((Vec4) yxy, y, x, y);
swizzle!((Vec4) yxz, y, x, z);
swizzle!((Vec4) yxw, y, x, w);
swizzle!((Vec4) yyx, y, y, x);
swizzle!((Vec4) yyy, y, y, y);
swizzle!((Vec4) yyz, y, y, z);
swizzle!((Vec4) yyw, y, y, w);
swizzle!((Vec4) yzx, y, z, x);
swizzle!((Vec4) yzy, y, z, y);
swizzle!((Vec4) yzz, y, z, z);
swizzle!((Vec4) yzw, y, z, w);
swizzle!((Vec4) ywx, y, w, x);
swizzle!((Vec4) ywy, y, w, y);
swizzle!((Vec4) ywz, y, w, z);
swizzle!((Vec4) yww, y, w, w);
swizzle!((Vec4) zxx, z, x, x);
swizzle!((Vec4) zxy, z, x, y);
swizzle!((Vec4) zxz, z, x, z);
swizzle!((Vec4) zxw, z, x, w);
swizzle!((Vec4) zyx, z, y, x);
swizzle!((Vec4) zyy, z, y, y);
swizzle!((Vec4) zyz, z, y, z);
swizzle!((Vec4) zyw, z, y, w);
swizzle!((Vec4) zzx, z, z, x);
swizzle!((Vec4) zzy, z, z, y);
swizzle!((Vec4) zzz, z, z, z);
swizzle!((Vec4) zzw, z, z, w);
swizzle!((Vec4) zwx, z, w, x);
swizzle!((Vec4) zwy, z, w, y);
swizzle!((Vec4) zwz, z, w, z);
swizzle!((Vec4) zww, z, w, w);
swizzle!((Vec4) wxx, w, x, x);
swizzle!((Vec4) wxy, w, x, y);
swizzle!((Vec4) wxz, w, x, z);
swizzle!((Vec4) wxw, w, x, w);
swizzle!((Vec4) wyx, w, y, x);
swizzle!((Vec4) wyy, w, y, y);
swizzle!((Vec4) wyz, w, y, z);
swizzle!((Vec4) wyw, w, y, w);
swizzle!((Vec4) wzx, w, z, x);
swizzle!((Vec4) wzy, w, z, y);
swizzle!((Vec4) wzz, w, z, z);
swizzle!((Vec4) wzw, w, z, w);
swizzle!((Vec4) wwx, w, w, x);
swizzle!((Vec4) wwy, w, w, y);
swizzle!((Vec4) wwz, w, w, z);
swizzle!((Vec4) www, w, w, w);
swizzle!((Vec4) xxxx, x, x, x, x);
swizzle!((Vec4) xxxy, x, x, x, y);
swizzle!((Vec4) xxxz, x, x, x, z);
swizzle!((Vec4) xxxw, x, x, x, w);
swizzle!((Vec4) xxyx, x, x, y, x);
swizzle!((Vec4) xxyy, x, x, y, y);
swizzle!((Vec4) xxyz, x, x, y, z);
swizzle!((Vec4) xxyw, x, x, y, w);
swizzle!((Vec4) xxzx, x, x, z, x);
swizzle!((Vec4) xxzy, x, x, z, y);
swizzle!((Vec4) xxzz, x, x, z, z);
swizzle!((Vec4) xxzw, x, x, z, w);
swizzle!((Vec4) xxwx, x, x, w, x);
swizzle!((Vec4) xxwy, x, x, w, y);
swizzle!((Vec4) xxwz, x, x, w, z);
swizzle!((Vec4) xxww, x, x, w, w);
swizzle!((Vec4) xyxx, x, y, x, x);
swizzle!((Vec4) xyxy, x, y, x, y);
swizzle!((Vec4) xyxz, x, y, x, z);
swizzle!((Vec4) xyxw, x, y, x, w);
swizzle!((Vec4) xyyx, x, y, y, x);
swizzle!((Vec4) xyyy, x, y, y, y);
swizzle!((Vec4) xyyz, x, y, y, z);
swizzle!((Vec4) xyyw, x, y, y, w);
swizzle!((Vec4) xyzx, x, y, z, x);
swizzle!((Vec4) xyzy, x, y, z, y);
swizzle!((Vec4) xyzz, x, y, z, z);
swizzle!((Vec4) xyzw, x, y, z, w);
swizzle!((Vec4) xywx, x, y, w, x);
swizzle!((Vec4) xywy, x, y, w, y);
swizzle!((Vec4) xywz, x, y, w, z);
swizzle!((Vec4) xyww, x, y, w, w);
swizzle!((Vec4) xzxx, x, z, x, x);
swizzle!((Vec4) xzxy, x, z, x, y);
swizzle!((Vec4) xzxz, x, z, x, z);
swizzle!((Vec4) xzxw, x, z, x, w);
swizzle!((Vec4) xzyx, x, z, y, x);
swizzle!((Vec4) xzyy, x, z, y, y);
swizzle!((Vec4) xzyz, x, z, y, z);
swizzle!((Vec4) xzyw, x, z, y, w);
swizzle!((Vec4) xzzx, x, z, z, x);
swizzle!((Vec4) xzzy, x, z, z, y);
swizzle!((Vec4) xzzz, x, z, z, z);
swizzle!((Vec4) xzzw, x, z, z, w);
swizzle!((Vec4) xzwx, x, z, w, x);
swizzle!((Vec4) xzwy, x, z, w, y);
swizzle!((Vec4) xzwz, x, z, w, z);
swizzle!((Vec4) xzww, x, z, w, w);
swizzle!((Vec4) xwxx, x, w, x, x);
swizzle!((Vec4) xwxy, x, w, x, y);
swizzle!((Vec4) xwxz, x, w, x, z);
swizzle!((Vec4) xwxw, x, w, x, w);
swizzle!((Vec4) xwyx, x, w, y, x);
swizzle!((Vec4) xwyy, x, w, y, y);
swizzle!((Vec4) xwyz, x, w, y, z);
swizzle!((Vec4) xwyw, x, w, y, w);
swizzle!((Vec4) xwzx, x, w, z, x);
swizzle!((Vec4) xwzy, x, w, z, y);
swizzle!((Vec4) xwzz, x, w, z, z);
swizzle!((Vec4) xwzw, x, w, z, w);
swizzle!((Vec4) xwwx, x, w, w, x);
swizzle!((Vec4) xwwy, x, w, w, y);
swizzle!((Vec4) xwwz, x, w, w, z);
swizzle!((Vec4) xwww, x, w, w, w);
swizzle!((Vec4) yxxx, y, x, x, x);
swizzle!((Vec4) yxxy, y, x, x, y);
swizzle!((Vec4) yxxz, y, x, x, z);
swizzle!((Vec4) yxxw, y, x, x, w);
swizzle!((Vec4) yxyx, y, x, y, x);
swizzle!((Vec4) yxyy, y, x, y, y);
swizzle!((Vec4) yxyz, y, x, y, z);
swizzle!((Vec4) yxyw, y, x, y, w);
swizzle!((Vec4) yxzx, y, x, z, x);
swizzle!((Vec4) yxzy, y, x, z, y);
swizzle!((Vec4) yxzz, y, x, z, z);
swizzle!((Vec4) yxzw, y, x, z, w);
swizzle!((Vec4) yxwx, y, x, w, x);
swizzle!((Vec4) yxwy, y, x, w, y);
swizzle!((Vec4) yxwz, y, x, w, z);
swizzle!((Vec4) yxww, y, x, w, w);
swizzle!((Vec4) yyxx, y, y, x, x);
swizzle!((Vec4) yyxy, y, y, x, y);
swizzle!((Vec4) yyxz, y, y, x, z);
swizzle!((Vec4) yyxw, y, y, x, w);
swizzle!((Vec4) yyyx, y, y, y, x);
swizzle!((Vec4) yyyy, y, y, y, y);
swizzle!((Vec4) yyyz, y, y, y, z);
swizzle!((Vec4) yyyw, y, y, y, w);
swizzle!((Vec4) yyzx, y, y, z, x);
swizzle!((Vec4) yyzy, y, y, z, y);
swizzle!((Vec4) yyzz, y, y, z, z);
swizzle!((Vec4) yyzw, y, y, z, w);
swizzle!((Vec4) yywx, y, y, w, x);
swizzle!((Vec4) yywy, y, y, w, y);
swizzle!((Vec4) yywz, y, y, w, z);
swizzle!((Vec4) yyww, y, y, w, w);
swizzle!((Vec4) yzxx, y, z, x, x);
swizzle!((Vec4) yzxy, y, z, x, y);
swizzle!((Vec4) yzxz, y, z, x, z);
swizzle!((Vec4) yzxw, y, z, x, w);
swizzle!((Vec4) yzyx, y, z, y, x);
swizzle!((Vec4) yzyy, y, z, y, y);
swizzle!((Vec4) yzyz, y, z, y, z);
swizzle!((Vec4) yzyw, y, z, y, w);
swizzle!((Vec4) yzzx, y, z, z, x);
swizzle!((Vec4) yzzy, y, z, z, y);
swizzle!((Vec4) yzzz, y, z, z, z);
swizzle!((Vec4) yzzw, y, z, z, w);
swizzle!((Vec4) yzwx, y, z, w, x);
swizzle!((Vec4) yzwy, y, z, w, y);
swizzle!((Vec4) yzwz, y, z, w, z);
swizzle!((Vec4) yzww, y, z, w, w);
swizzle!((Vec4) ywxx, y, w, x, x);
swizzle!((Vec4) ywxy, y, w, x, y);
swizzle!((Vec4) ywxz, y, w, x, z);
swizzle!((Vec4) ywxw, y, w, x, w);
swizzle!((Vec4) ywyx, y, w, y, x);
swizzle!((Vec4) ywyy, y, w, y, y);
swizzle!((Vec4) ywyz, y, w, y, z);
swizzle!((Vec4) ywyw, y, w, y, w);
swizzle!((Vec4) ywzx, y, w, z, x);
swizzle!((Vec4) ywzy, y, w, z, y);
swizzle!((Vec4) ywzz, y, w, z, z);
swizzle!((Vec4) ywzw, y, w, z, w);
swizzle!((Vec4) ywwx, y, w, w, x);
swizzle!((Vec4) ywwy, y, w, w, y);
swizzle!((Vec4) ywwz, y, w, w, z);
swizzle!((Vec4) ywww, y, w, w, w);
swizzle!((Vec4) zxxx, z, x, x, x);
swizzle!((Vec4) zxxy, z, x, x, y);
swizzle!((Vec4) zxxz, z, x, x, z);
swizzle!((Vec4) zxxw, z, x, x, w);
swizzle!((Vec4) zxyx, z, x, y, x);
swizzle!((Vec4) zxyy, z, x, y, y);
swizzle!((Vec4) zxyz, z, x, y, z);
swizzle!((Vec4) zxyw, z, x, y, w);
swizzle!((Vec4) zxzx, z, x, z, x);
swizzle!((Vec4) zxzy, z, x, z, y);
swizzle!((Vec4) zxzz, z, x, z, z);
swizzle!((Vec4) zxzw, z, x, z, w);
swizzle!((Vec4) zxwx, z, x, w, x);
swizzle!((Vec4) zxwy, z, x, w, y);
swizzle!((Vec4) zxwz, z, x, w, z);
swizzle!((Vec4) zxww, z, x, w, w);
swizzle!((Vec4) zyxx, z, y, x, x);
swizzle!((Vec4) zyxy, z, y, x, y);
swizzle!((Vec4) zyxz, z, y, x, z);
swizzle!((Vec4) zyxw, z, y, x, w);
swizzle!((Vec4) zyyx, z, y, y, x);
swizzle!((Vec4) zyyy, z, y, y, y);
swizzle!((Vec4) zyyz, z, y, y, z);
swizzle!((Vec4) zyyw, z, y, y, w);
swizzle!((Vec4) zyzx, z, y, z, x);
swizzle!((Vec4) zyzy, z, y, z, y);
swizzle!((Vec4) zyzz, z, y, z, z);
swizzle!((Vec4) zyzw, z, y, z, w);
swizzle!((Vec4) zywx, z, y, w, x);
swizzle!((Vec4) zywy, z, y, w, y);
swizzle!((Vec4) zywz, z, y, w, z);
swizzle!((Vec4) zyww, z, y, w, w);
swizzle!((Vec4) zzxx, z, z, x, x);
swizzle!((Vec4) zzxy, z, z, x, y);
swizzle!((Vec4) zzxz, z, z, x, z);
swizzle!((Vec4) zzxw, z, z, x, w);
swizzle!((Vec4) zzyx, z, z, y, x);
swizzle!((Vec4) zzyy, z, z, y, y);
swizzle!((Vec4) zzyz, z, z, y, z);
swizzle!((Vec4) zzyw, z, z, y, w);
swizzle!((Vec4) zzzx, z, z, z, x);
swizzle!((Vec4) zzzy, z, z, z, y);
swizzle!((Vec4) zzzz, z, z, z, z);
swizzle!((Vec4) zzzw, z, z, z, w);
swizzle!((Vec4) zzwx, z, z, w, x);
swizzle!((Vec4) zzwy, z, z, w, y);
swizzle!((Vec4) zzwz, z, z, w, z);
swizzle!((Vec4) zzww, z, z, w, w);
swizzle!((Vec4) zwxx, z, w, x, x);
swizzle!((Vec4) zwxy, z, w, x, y);
swizzle!((Vec4) zwxz, z, w, x, z);
swizzle!((Vec4) zwxw, z, w, x, w);
swizzle!((Vec4) zwyx, z, w, y, x);
swizzle!((Vec4) zwyy, z, w, y, y);
swizzle!((Vec4) zwyz, z, w, y, z);
swizzle!((Vec4) zwyw, z, w, y, w);
swizzle!((Vec4) zwzx, z, w, z, x);
swizzle!((Vec4) zwzy, z, w, z, y);
swizzle!((Vec4) zwzz, z, w, z, z);
swizzle!((Vec4) zwzw, z, w, z, w);
swizzle!((Vec4) zwwx, z, w, w, x);
swizzle!((Vec4) zwwy, z, w, w, y);
swizzle!((Vec4) zwwz, z, w, w, z);
swizzle!((Vec4) zwww, z, w, w, w);
swizzle!((Vec4) wxxx, w, x, x, x);
swizzle!((Vec4) wxxy, w, x, x, y);
swizzle!((Vec4) wxxz, w, x, x, z);
swizzle!((Vec4) wxxw, w, x, x, w);
swizzle!((Vec4) wxyx, w, x, y, x);
swizzle!((Vec4) wxyy, w, x, y, y);
swizzle!((Vec4) wxyz, w, x, y, z);
swizzle!((Vec4) wxyw, w, x, y, w);
swizzle!((Vec4) wxzx, w, x, z, x);
swizzle!((Vec4) wxzy, w, x, z, y);
swizzle!((Vec4) wxzz, w, x, z, z);
swizzle!((Vec4) wxzw, w, x, z, w);
swizzle!((Vec4) wxwx, w, x, w, x);
swizzle!((Vec4) wxwy, w, x, w, y);
swizzle!((Vec4) wxwz, w, x, w, z);
swizzle!((Vec4) wxww, w, x, w, w);
swizzle!((Vec4) wyxx, w, y, x, x);
swizzle!((Vec4) wyxy, w, y, x, y);
swizzle!((Vec4) wyxz, w, y, x, z);
swizzle!((Vec4) wyxw, w, y, x, w);
swizzle!((Vec4) wyyx, w, y, y, x);
swizzle!((Vec4) wyyy, w, y, y, y);
swizzle!((Vec4) wyyz, w, y, y, z);
swizzle!((Vec4) wyyw, w, y, y, w);
swizzle!((Vec4) wyzx, w, y, z, x);
swizzle!((Vec4) wyzy, w, y, z, y);
swizzle!((Vec4) wyzz, w, y, z, z);
swizzle!((Vec4) wyzw, w, y, z, w);
swizzle!((Vec4) wywx, w, y, w, x);
swizzle!((Vec4) wywy, w, y, w, y);
swizzle!((Vec4) wywz, w, y, w, z);
swizzle!((Vec4) wyww, w, y, w, w);
swizzle!((Vec4) wzxx, w, z, x, x);
swizzle!((Vec4) wzxy, w, z, x, y);
swizzle!((Vec4) wzxz, w, z, x, z);
swizzle!((Vec4) wzxw, w, z, x, w);
swizzle!((Vec4) wzyx, w, z, y, x);
swizzle!((Vec4) wzyy, w, z, y, y);
swizzle!((Vec4) wzyz, w, z, y, z);
swizzle!((Vec4) wzyw, w, z, y, w);
swizzle!((Vec4) wzzx, w, z, z, x);
swizzle!((Vec4) wzzy, w, z, z, y);
swizzle!((Vec4) wzzz, w, z, z, z);
swizzle!((Vec4) wzzw, w, z, z, w);
swizzle!((Vec4) wzwx, w, z, w, x);
swizzle!((Vec4) wzwy, w, z, w, y);
swizzle!((Vec4) wzwz, w, z, w, z);
swizzle!((Vec4) wzww, w, z, w, w);
swizzle!((Vec4) wwxx, w, w, x, x);
swizzle!((Vec4) wwxy, w, w, x, y);
swizzle!((Vec4) wwxz, w, w, x, z);
swizzle!((Vec4) wwxw, w, w, x, w);
swizzle!((Vec4) wwyx, w, w, y, x);
swizzle!((Vec4) wwyy, w, w, y, y);
swizzle!((Vec4) wwyz, w, w, y, z);
swizzle!((Vec4) wwyw, w, w, y, w);
swizzle!((Vec4) wwzx, w, w, z, x);
swizzle!((Vec4) wwzy, w, w, z, y);
swizzle!((Vec4) wwzz, w, w, z, z);
swizzle!((Vec4) wwzw, w, w, z, w);
swizzle!((Vec4) wwwx, w, w, w, x);
swizzle!((Vec4) wwwy, w, w, w, y);
swizzle!((Vec4) wwwz, w, w, w, z);
swizzle!((Vec4) wwww, w, w, w, w);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_with_new() {
        let vec = Vec4 {
            x: 42.0,
            y: -69.0,
            z: 666.6,
            w: 420.0
        };
        let vec_from_new = Vec4::new(42.0, -69.0, 666.6, 420.0);
        assert_eq!(vec, vec_from_new);
    }

    #[test]
    fn create_from_macro_one() {
        let vec = Vec4::new(3.0, 3.0, 3.0, 3.0);
        let vec_from_macro = vec4!(3);
        assert_eq!(vec, vec_from_macro);
    }

    #[test]
    fn create_from_macro_four() {
        let vec = Vec4::new(-1.0, 0.0, 1.0, 0.0);
        let vec_from_macro = vec4!(-1, 0, 1, 0);
        assert_eq!(vec, vec_from_macro);
    }

    #[test]
    fn len() {
        let vec = vec4!(2, 3, 8, 2);
        assert_eq!(vec.len(), 9.0);
    }

    #[test]
    fn normalize() {
        let vec = vec4!(1, 2, 1, 2);
        assert_eq!(vec.normalized().len(), 1.0);
    }

    #[test]
    fn dot() {
        let vec1 = vec4!(1, 2, 8, -5);
        let vec2 = vec4!(-5, -2, 3, 3);
        assert_eq!(vec1.dot(&vec2), 0.0);
        assert_eq!(vec1.dot(&vec1).sqrt(), vec1.len());
    }

    generate_op_test!(add_vec; vec4!(1), +, vec4!(2) => vec4!(3));
    generate_op_test!(add_float; vec4!(1), +, 0.2 => vec4!(1.2));
    generate_op_test!((assign) add_assign_vecs; vec4!(1), +=, vec4!(2) => vec4!(3));
    generate_op_test!((assign) add_assign_float; vec4!(1), +=, 68.0 => vec4!(69));

    generate_op_test!(sub_vec; vec4!(42), -, vec4!(12) => vec4!(30));
    generate_op_test!(sub_float; vec4!(39), -, 18.0 => vec4!(21));
    generate_op_test!((assign) sub_assign_vec; vec4!(44), -=, vec4!(26) => vec4!(18));
    generate_op_test!((assign) sub_assign_float; vec4!(97), -=, 82.0 => vec4!(15));

    generate_op_test!(mul_vec; vec4!(8), *, vec4!(4) => vec4!(32));
    generate_op_test!(mul_float; vec4!(10), *, 2.5 => vec4!(25));
    generate_op_test!((assign) mul_assign_vec; vec4!(44), *=, vec4!(0.25) => vec4!(11));
    generate_op_test!((assign) mul_assign_float; vec4!(13), *=, 3.0 => vec4!(39));

    generate_op_test!(div_vec; vec4!(42), /, vec4!(21) => vec4!(2));
    generate_op_test!(div_float; vec4!(39), /, 13.0 => vec4!(3));
    generate_op_test!((assign) div_assign_vec; vec4!(44), /=, vec4!(4) => vec4!(11));
    generate_op_test!((assign) div_assign_float; vec4!(99), /=, 3.0 => vec4!(33));

    #[test]
    fn neg() {
        let vec = vec4!(1, -2, 3, -4);
        assert_eq!(-vec, vec4!(-1, 2, -3, 4));
    }
}