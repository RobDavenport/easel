use crate::float::Float;

/// Trait for types that can be linearly interpolated.
pub trait Lerp<F: Float> {
    /// Linearly interpolate between `self` and `other` by factor `t`.
    fn lerp(&self, other: &Self, t: F) -> Self;
}

impl<F: Float> Lerp<F> for F {
    fn lerp(&self, other: &Self, t: F) -> Self {
        *self + (*other - *self) * t
    }
}

impl<F: Float> Lerp<F> for (F, F) {
    fn lerp(&self, other: &Self, t: F) -> Self {
        (
            Float::lerp(self.0, other.0, t),
            Float::lerp(self.1, other.1, t),
        )
    }
}

impl<F: Float> Lerp<F> for (F, F, F) {
    fn lerp(&self, other: &Self, t: F) -> Self {
        (
            Float::lerp(self.0, other.0, t),
            Float::lerp(self.1, other.1, t),
            Float::lerp(self.2, other.2, t),
        )
    }
}

impl<F: Float> Lerp<F> for (F, F, F, F) {
    fn lerp(&self, other: &Self, t: F) -> Self {
        (
            Float::lerp(self.0, other.0, t),
            Float::lerp(self.1, other.1, t),
            Float::lerp(self.2, other.2, t),
            Float::lerp(self.3, other.3, t),
        )
    }
}

impl<F: Float, const N: usize> Lerp<F> for [F; N] {
    fn lerp(&self, other: &Self, t: F) -> Self {
        let mut result = *self;
        for i in 0..N {
            result[i] = Float::lerp(self[i], other[i], t);
        }
        result
    }
}

/// RGBA color with premultiplied alpha interpolation.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Rgba<F: Float> {
    pub r: F,
    pub g: F,
    pub b: F,
    pub a: F,
}

impl<F: Float> Rgba<F> {
    pub fn new(r: F, g: F, b: F, a: F) -> Self {
        Self { r, g, b, a }
    }
}

impl<F: Float> Lerp<F> for Rgba<F> {
    fn lerp(&self, other: &Self, t: F) -> Self {
        let pre_self = (self.r * self.a, self.g * self.a, self.b * self.a, self.a);
        let pre_other = (
            other.r * other.a,
            other.g * other.a,
            other.b * other.a,
            other.a,
        );

        let a = Float::lerp(pre_self.3, pre_other.3, t);
        if a <= F::zero() {
            return Self::new(F::zero(), F::zero(), F::zero(), F::zero());
        }

        let r = Float::lerp(pre_self.0, pre_other.0, t) / a;
        let g = Float::lerp(pre_self.1, pre_other.1, t) / a;
        let b = Float::lerp(pre_self.2, pre_other.2, t) / a;

        Self::new(r, g, b, a)
    }
}

/// Angle in radians with shortest-path interpolation.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Angle<F: Float> {
    pub radians: F,
}

impl<F: Float> Angle<F> {
    pub fn from_radians(radians: F) -> Self {
        Self { radians }
    }

    pub fn from_degrees(degrees: F) -> Self {
        Self {
            radians: degrees * F::pi() / F::from_f32(180.0),
        }
    }

    pub fn to_degrees(self) -> F {
        self.radians * F::from_f32(180.0) / F::pi()
    }
}

impl<F: Float> Lerp<F> for Angle<F> {
    fn lerp(&self, other: &Self, t: F) -> Self {
        let mut diff = other.radians - self.radians;
        let pi = F::pi();
        let tau = F::tau();

        while diff > pi {
            diff = diff - tau;
        }
        while diff < -pi {
            diff = diff + tau;
        }

        Self {
            radians: self.radians + diff * t,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Angle, Lerp, Rgba};

    const EPS: f32 = 1e-5;

    fn approx(a: f32, b: f32) -> bool {
        (a - b).abs() <= EPS
    }

    #[test]
    fn lerp_scalar_midpoint() {
        let a = 0.0f32;
        let b = 10.0f32;
        assert!(approx(a.lerp(&b, 0.5), 5.0));
    }

    #[test]
    fn lerp_scalar_endpoints() {
        let a = -4.0f32;
        let b = 6.0f32;
        assert!(approx(a.lerp(&b, 0.0), -4.0));
        assert!(approx(a.lerp(&b, 1.0), 6.0));
    }

    #[test]
    fn lerp_tuple2() {
        let a = (0.0f32, 0.0f32);
        let b = (10.0f32, 20.0f32);
        assert_eq!(a.lerp(&b, 0.5), (5.0, 10.0));
    }

    #[test]
    fn lerp_tuple3() {
        let a = (1.0f32, 2.0f32, 3.0f32);
        let b = (3.0f32, 6.0f32, 9.0f32);
        assert_eq!(a.lerp(&b, 0.5), (2.0, 4.0, 6.0));
    }

    #[test]
    fn lerp_tuple4() {
        let a = (0.0f32, 0.0f32, 0.0f32, 0.0f32);
        let b = (4.0f32, 8.0f32, 12.0f32, 16.0f32);
        assert_eq!(a.lerp(&b, 0.25), (1.0, 2.0, 3.0, 4.0));
    }

    #[test]
    fn lerp_array() {
        let a = [0.0f32; 4];
        let b = [4.0f32; 4];
        assert_eq!(a.lerp(&b, 0.25), [1.0f32; 4]);
    }

    #[test]
    fn lerp_rgba_opaque() {
        let a = Rgba::new(1.0f32, 0.0, 0.0, 1.0);
        let b = Rgba::new(0.0f32, 0.0, 1.0, 1.0);
        let mid = a.lerp(&b, 0.5);
        assert!(approx(mid.r, 0.5));
        assert!(approx(mid.b, 0.5));
        assert!(approx(mid.a, 1.0));
    }

    #[test]
    fn lerp_rgba_transparent() {
        let a = Rgba::new(1.0f32, 0.0, 0.0, 1.0);
        let b = Rgba::new(0.0f32, 0.0, 0.0, 0.0);
        let mid = a.lerp(&b, 0.5);
        assert!(mid.r > 0.95);
        assert!(approx(mid.a, 0.5));
    }

    fn wrap_degrees(v: f32) -> f32 {
        let mut x = v % 360.0;
        if x < 0.0 {
            x += 360.0;
        }
        x
    }

    #[test]
    fn lerp_angle_short_path() {
        let a = Angle::from_degrees(350.0f32);
        let b = Angle::from_degrees(10.0f32);
        let mid = a.lerp(&b, 0.5).to_degrees();
        assert!(approx(wrap_degrees(mid), 0.0));
    }

    #[test]
    fn lerp_angle_half() {
        let a = Angle::from_degrees(0.0f32);
        let b = Angle::from_degrees(180.0f32);
        let mid = a.lerp(&b, 0.5).to_degrees();
        assert!(approx(mid, 90.0));
    }
}
