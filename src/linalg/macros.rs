macro_rules! vec2 {
    ($x:expr, $y:expr) => {Vec2::new($x as f64, $y as f64)}; 
    ($one:expr) => {Vec2::new($one as f64, $one as f64)};
}

macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {Vec3::new($x as f64, $y as f64, $z as f64)};
    ($one:expr) => {Vec3::new($one as f64, $one as f64, $one as f64)};
    (v2 $xy:expr, $z:expr) => {Vec3::new($xy.x, $xy.y, $z as f64)};
    ($x:expr, v2 $yz:expr) => {Vec3::new($x as f64, $yz.x, $yz.y)};
}

macro_rules! vec4 {
    ($x:expr, $y:expr, $z:expr, $w:expr) => {Vec4::new($x as f64, $y as f64, $z as f64, $w as f64)};
    ($one:expr) => {Vec4::new($one as f64, $one as f64, $one as f64, $one as f64)};
    (v2 $xy:expr, $z:expr, $w:expr) => {Vec4::new($xy.x, $xy.y, $z as f64, $w as f64)};
    ($x:expr, v2 $yz:expr, $w:expr) => {Vec4::new($x as f64, $yz.x, $yz.y, $w as f64)};
    ($x:expr, $y:expr, v2 $zw:expr) => {Vec4::new($x as f64, $y as f64, $zw.x, $zw.y)};
    (v2 $xy:expr, v2 $zw:expr) => {Vec4::new($xy.x, $xy.y, $zw.x, $zw.y)};
    (v3 $xyz:expr, $w:expr) => {Vec4::new($xyz.x, $xyz.y, $xyz.z, $w as f64)};
    ($x:expr, v3 $yzw:expr) => {Vec4::new($x as f64, $yzw.x, $yzw.y, $yzw.z)};
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

macro_rules! swizzle {
    (($type:ty) $name:ident, $ax1:ident, $ax2:ident) => {
        impl $type {
            pub fn $name(&self) -> Vec2 {
                vec2!(self.$ax1, self.$ax2)
            }
        }
    };
    (($type:ty) $name:ident, $ax1:ident, $ax2:ident, $ax3:ident) => {
        impl $type {
            pub fn $name(&self) -> Vec3 {
                vec3!(self.$ax1, self.$ax2, self.$ax3)
            }
        }
    };
    (($type:ty) $name:ident, $ax1:ident, $ax2:ident, $ax3:ident, $ax4:ident) => {
        impl $type {
            pub fn $name(&self) -> Vec4 {
                vec4!(self.$ax1, self.$ax2, self.$ax3, self.$ax4)
            }
        }
    };
}

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