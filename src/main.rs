#[macro_use]
pub mod linalg {
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Vec3 {
        pub x: f64,
        pub y: f64,
        pub z: f64
    }

    macro_rules! vec3 {
        ($x:expr, $y:expr, $z:expr) => {Vec3::new($x as f64, $y as f64, $z as f64)}; 
        ($one:expr) => {vec3!($one, $one, $one)};
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
    }

    macro_rules! impl_vec_ops {
        ($type:ty; [$($var:ident),+]  ($op_trait:ident, $op_fn:ident, $op_assign_trait:ident, $op_assign_fn:ident => $op:tt)) => {

            impl $op_trait for $type {
                type Output = Self;
                fn $op_fn(self, rhs: Self) -> Self {
                    Self { $($var: self.$var $op rhs.$var),+ }
                }
            }

            impl $op_trait<f64> for $type {
                type Output = Self;
                fn $op_fn(self, rhs: f64) -> Self {
                    Self { $($var: self.$var $op rhs),+ }
                }
            }

            impl $op_assign_trait for $type {
                fn $op_assign_fn(&mut self, rhs: Self) {
                    *self = *self $op rhs;
                }
            }

            impl $op_assign_trait<f64> for $type {
                fn $op_assign_fn(&mut self, rhs: f64) {
                    *self = *self $op rhs;
                }
            }

        };
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

        // TODO tests for dot and cross

        macro_rules! generate_op_test {
            ($name:ident; $lhs:expr, $op:tt, $rhs:expr => $ans:expr) => {
                #[test]
                fn $name() {
                    assert_eq!($lhs $op $rhs, $ans);
                }
            };
            ((assign) $name:ident; $lhs:expr, $op:tt, $rhs:expr => $ans:expr) => {
                #[test]
                fn $name() {
                    let mut lhs = $lhs;
                    lhs $op $rhs;
                    assert_eq!(lhs, $ans);
                }
            };
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

        // TODO test Neg
    }
}

fn main() {
    use linalg::Vec3;
    let forward = vec3!(0, 0, -1);
    let up = vec3!(0, 0.6, 0.8);
    println!("{}", up.len());
    let right = forward.cross(&up).normalized();
    println!("{:?}", right);
    let up = right.cross(&forward).normalized();
    println!("{:?}", up);
}