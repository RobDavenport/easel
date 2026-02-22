/// Trait abstracting floating-point operations for animation math.
///
/// Implemented for `f32` and `f64`. Uses `libm` for no_std transcendental functions.
pub trait Float:
    Copy
    + Clone
    + PartialEq
    + PartialOrd
    + core::ops::Add<Output = Self>
    + core::ops::Sub<Output = Self>
    + core::ops::Mul<Output = Self>
    + core::ops::Div<Output = Self>
    + core::ops::Neg<Output = Self>
    + Default
    + core::fmt::Debug
{
    fn zero() -> Self;
    fn one() -> Self;
    fn half() -> Self;
    fn two() -> Self;
    fn pi() -> Self;
    fn tau() -> Self;
    fn from_f32(v: f32) -> Self;
    fn to_f32(self) -> f32;
    fn sqrt(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn abs(self) -> Self;
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
    fn powf(self, exp: Self) -> Self;
    fn exp(self) -> Self;
    fn floor(self) -> Self;

    fn clamp(self, min: Self, max: Self) -> Self {
        self.max(min).min(max)
    }

    fn lerp(self, other: Self, t: Self) -> Self {
        self + (other - self) * t
    }

    fn remap01(self, from_min: Self, from_max: Self) -> Self {
        (self - from_min) / (from_max - from_min)
    }
}

impl Float for f32 {
    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }

    fn half() -> Self {
        0.5
    }

    fn two() -> Self {
        2.0
    }

    fn pi() -> Self {
        core::f32::consts::PI
    }

    fn tau() -> Self {
        core::f32::consts::TAU
    }

    fn from_f32(v: f32) -> Self {
        v
    }

    fn to_f32(self) -> f32 {
        self
    }

    fn sqrt(self) -> Self {
        libm::sqrtf(self)
    }

    fn sin(self) -> Self {
        libm::sinf(self)
    }

    fn cos(self) -> Self {
        libm::cosf(self)
    }

    fn abs(self) -> Self {
        libm::fabsf(self)
    }

    fn min(self, other: Self) -> Self {
        if self < other {
            self
        } else {
            other
        }
    }

    fn max(self, other: Self) -> Self {
        if self > other {
            self
        } else {
            other
        }
    }

    fn powf(self, exp: Self) -> Self {
        libm::powf(self, exp)
    }

    fn exp(self) -> Self {
        libm::expf(self)
    }

    fn floor(self) -> Self {
        libm::floorf(self)
    }
}

impl Float for f64 {
    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }

    fn half() -> Self {
        0.5
    }

    fn two() -> Self {
        2.0
    }

    fn pi() -> Self {
        core::f64::consts::PI
    }

    fn tau() -> Self {
        core::f64::consts::TAU
    }

    fn from_f32(v: f32) -> Self {
        v as f64
    }

    fn to_f32(self) -> f32 {
        self as f32
    }

    fn sqrt(self) -> Self {
        libm::sqrt(self)
    }

    fn sin(self) -> Self {
        libm::sin(self)
    }

    fn cos(self) -> Self {
        libm::cos(self)
    }

    fn abs(self) -> Self {
        libm::fabs(self)
    }

    fn min(self, other: Self) -> Self {
        if self < other {
            self
        } else {
            other
        }
    }

    fn max(self, other: Self) -> Self {
        if self > other {
            self
        } else {
            other
        }
    }

    fn powf(self, exp: Self) -> Self {
        libm::pow(self, exp)
    }

    fn exp(self) -> Self {
        libm::exp(self)
    }

    fn floor(self) -> Self {
        libm::floor(self)
    }
}

#[cfg(test)]
mod tests {
    use super::Float;

    const EPS_F32: f32 = 1e-6;
    const EPS_F64: f64 = 1e-10;

    #[test]
    fn float_f32_basics() {
        assert_eq!(f32::zero(), 0.0);
        assert_eq!(f32::one(), 1.0);
        assert_eq!(f32::half(), 0.5);
        let v = 3.25f32;
        assert!((f32::from_f32(v).to_f32() - v).abs() < EPS_F32);
    }

    #[test]
    fn float_f32_math() {
        assert!((f32::from_f32(4.0).sqrt() - 2.0).abs() < EPS_F32);
        assert!(f32::zero().sin().abs() < EPS_F32);
        assert!((f32::zero().cos() - 1.0).abs() < EPS_F32);
    }

    #[test]
    fn float_f64_basics() {
        assert_eq!(f64::zero(), 0.0);
        assert_eq!(f64::one(), 1.0);
        assert_eq!(f64::half(), 0.5);
        let v = 8.5f32;
        assert!((f64::from_f32(v).to_f32() - v).abs() < 1e-6);
    }

    #[test]
    fn float_f64_math() {
        assert!((f64::from_f32(4.0).sqrt() - 2.0).abs() < EPS_F64);
        assert!(f64::zero().sin().abs() < EPS_F64);
        assert!((f64::zero().cos() - 1.0).abs() < EPS_F64);
    }
}
