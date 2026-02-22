use crate::float::Float;

/// All standard easing functions plus cubic Bezier.
#[derive(Clone, Debug, PartialEq)]
pub enum Easing<F: Float> {
    Linear,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInQuart,
    EaseOutQuart,
    EaseInOutQuart,
    EaseInQuint,
    EaseOutQuint,
    EaseInOutQuint,
    EaseInSine,
    EaseOutSine,
    EaseInOutSine,
    EaseInExpo,
    EaseOutExpo,
    EaseInOutExpo,
    EaseInCirc,
    EaseOutCirc,
    EaseInOutCirc,
    EaseInBack,
    EaseOutBack,
    EaseInOutBack,
    EaseInElastic,
    EaseOutElastic,
    EaseInOutElastic,
    EaseInBounce,
    EaseOutBounce,
    EaseInOutBounce,
    CubicBezier { x1: F, y1: F, x2: F, y2: F },
}

impl<F: Float> Easing<F> {
    /// Evaluate the easing function at time `t`.
    pub fn evaluate(&self, t: F) -> F {
        match self {
            Self::Linear => t,
            Self::EaseInQuad => ease_in_quad(t),
            Self::EaseOutQuad => ease_out_quad(t),
            Self::EaseInOutQuad => ease_in_out_quad(t),
            Self::EaseInCubic => ease_in_cubic(t),
            Self::EaseOutCubic => ease_out_cubic(t),
            Self::EaseInOutCubic => ease_in_out_cubic(t),
            Self::EaseInQuart => ease_in_quart(t),
            Self::EaseOutQuart => ease_out_quart(t),
            Self::EaseInOutQuart => ease_in_out_quart(t),
            Self::EaseInQuint => ease_in_quint(t),
            Self::EaseOutQuint => ease_out_quint(t),
            Self::EaseInOutQuint => ease_in_out_quint(t),
            Self::EaseInSine => ease_in_sine(t),
            Self::EaseOutSine => ease_out_sine(t),
            Self::EaseInOutSine => ease_in_out_sine(t),
            Self::EaseInExpo => ease_in_expo(t),
            Self::EaseOutExpo => ease_out_expo(t),
            Self::EaseInOutExpo => ease_in_out_expo(t),
            Self::EaseInCirc => ease_in_circ(t),
            Self::EaseOutCirc => ease_out_circ(t),
            Self::EaseInOutCirc => ease_in_out_circ(t),
            Self::EaseInBack => ease_in_back(t),
            Self::EaseOutBack => ease_out_back(t),
            Self::EaseInOutBack => ease_in_out_back(t),
            Self::EaseInElastic => ease_in_elastic(t),
            Self::EaseOutElastic => ease_out_elastic(t),
            Self::EaseInOutElastic => ease_in_out_elastic(t),
            Self::EaseInBounce => ease_in_bounce(t),
            Self::EaseOutBounce => ease_out_bounce(t),
            Self::EaseInOutBounce => ease_in_out_bounce(t),
            Self::CubicBezier { x1, y1, x2, y2 } => cubic_bezier(t, *x1, *y1, *x2, *y2),
        }
    }
}

pub fn ease_in_quad<F: Float>(t: F) -> F {
    t * t
}

pub fn ease_out_quad<F: Float>(t: F) -> F {
    let u = F::one() - t;
    F::one() - u * u
}

pub fn ease_in_out_quad<F: Float>(t: F) -> F {
    if t < F::half() {
        F::two() * t * t
    } else {
        let u = F::from_f32(-2.0) * t + F::two();
        F::one() - u * u / F::two()
    }
}

pub fn ease_in_cubic<F: Float>(t: F) -> F {
    t * t * t
}

pub fn ease_out_cubic<F: Float>(t: F) -> F {
    let u = F::one() - t;
    F::one() - u * u * u
}

pub fn ease_in_out_cubic<F: Float>(t: F) -> F {
    if t < F::half() {
        F::from_f32(4.0) * t * t * t
    } else {
        let u = F::from_f32(-2.0) * t + F::two();
        F::one() - u * u * u / F::two()
    }
}

pub fn ease_in_quart<F: Float>(t: F) -> F {
    t * t * t * t
}

pub fn ease_out_quart<F: Float>(t: F) -> F {
    let u = F::one() - t;
    F::one() - u * u * u * u
}

pub fn ease_in_out_quart<F: Float>(t: F) -> F {
    if t < F::half() {
        F::from_f32(8.0) * t * t * t * t
    } else {
        let u = F::from_f32(-2.0) * t + F::two();
        F::one() - u * u * u * u / F::two()
    }
}

pub fn ease_in_quint<F: Float>(t: F) -> F {
    t * t * t * t * t
}

pub fn ease_out_quint<F: Float>(t: F) -> F {
    let u = F::one() - t;
    F::one() - u * u * u * u * u
}

pub fn ease_in_out_quint<F: Float>(t: F) -> F {
    if t < F::half() {
        F::from_f32(16.0) * t * t * t * t * t
    } else {
        let u = F::from_f32(-2.0) * t + F::two();
        F::one() - u * u * u * u * u / F::two()
    }
}

pub fn ease_in_sine<F: Float>(t: F) -> F {
    F::one() - (t * F::pi() / F::two()).cos()
}

pub fn ease_out_sine<F: Float>(t: F) -> F {
    (t * F::pi() / F::two()).sin()
}

pub fn ease_in_out_sine<F: Float>(t: F) -> F {
    -((F::pi() * t).cos() - F::one()) / F::two()
}

pub fn ease_in_expo<F: Float>(t: F) -> F {
    if t == F::zero() {
        F::zero()
    } else {
        F::two().powf(F::from_f32(10.0) * t - F::from_f32(10.0))
    }
}

pub fn ease_out_expo<F: Float>(t: F) -> F {
    if t == F::one() {
        F::one()
    } else {
        F::one() - F::two().powf(F::from_f32(-10.0) * t)
    }
}

pub fn ease_in_out_expo<F: Float>(t: F) -> F {
    if t == F::zero() {
        F::zero()
    } else if t == F::one() {
        F::one()
    } else if t < F::half() {
        F::two().powf(F::from_f32(20.0) * t - F::from_f32(10.0)) / F::two()
    } else {
        (F::two() - F::two().powf(F::from_f32(-20.0) * t + F::from_f32(10.0))) / F::two()
    }
}

pub fn ease_in_circ<F: Float>(t: F) -> F {
    F::one() - (F::one() - t * t).sqrt()
}

pub fn ease_out_circ<F: Float>(t: F) -> F {
    let u = t - F::one();
    (F::one() - u * u).sqrt()
}

pub fn ease_in_out_circ<F: Float>(t: F) -> F {
    if t < F::half() {
        let u = F::two() * t;
        (F::one() - (F::one() - u * u).sqrt()) / F::two()
    } else {
        let u = F::from_f32(-2.0) * t + F::two();
        ((F::one() - u * u).sqrt() + F::one()) / F::two()
    }
}

pub fn ease_in_back<F: Float>(t: F) -> F {
    let c1 = F::from_f32(1.70158);
    let c3 = c1 + F::one();
    c3 * t * t * t - c1 * t * t
}

pub fn ease_out_back<F: Float>(t: F) -> F {
    let c1 = F::from_f32(1.70158);
    let c3 = c1 + F::one();
    let u = t - F::one();
    F::one() + c3 * u * u * u + c1 * u * u
}

pub fn ease_in_out_back<F: Float>(t: F) -> F {
    let c1 = F::from_f32(1.70158);
    let c2 = c1 * F::from_f32(1.525);
    if t < F::half() {
        let u = F::two() * t;
        (u * u * ((c2 + F::one()) * u - c2)) / F::two()
    } else {
        let u = F::two() * t - F::two();
        (u * u * ((c2 + F::one()) * u + c2) + F::two()) / F::two()
    }
}

pub fn ease_in_elastic<F: Float>(t: F) -> F {
    if t == F::zero() || t == F::one() {
        return t;
    }
    let c4 = F::tau() / F::from_f32(3.0);
    -(F::two().powf(F::from_f32(10.0) * t - F::from_f32(10.0)))
        * ((F::from_f32(10.0) * t - F::from_f32(10.75)) * c4).sin()
}

pub fn ease_out_elastic<F: Float>(t: F) -> F {
    if t == F::zero() || t == F::one() {
        return t;
    }
    let c4 = F::tau() / F::from_f32(3.0);
    F::two().powf(F::from_f32(-10.0) * t) * ((F::from_f32(10.0) * t - F::from_f32(0.75)) * c4).sin()
        + F::one()
}

pub fn ease_in_out_elastic<F: Float>(t: F) -> F {
    if t == F::zero() || t == F::one() {
        return t;
    }
    let c5 = F::tau() / F::from_f32(4.5);
    if t < F::half() {
        -(F::two().powf(F::from_f32(20.0) * t - F::from_f32(10.0))
            * ((F::from_f32(20.0) * t - F::from_f32(11.125)) * c5).sin())
            / F::two()
    } else {
        (F::two().powf(F::from_f32(-20.0) * t + F::from_f32(10.0))
            * ((F::from_f32(20.0) * t - F::from_f32(11.125)) * c5).sin())
            / F::two()
            + F::one()
    }
}

pub fn ease_in_bounce<F: Float>(t: F) -> F {
    F::one() - ease_out_bounce(F::one() - t)
}

pub fn ease_out_bounce<F: Float>(t: F) -> F {
    let n1 = F::from_f32(7.5625);
    let d1 = F::from_f32(2.75);
    if t < F::one() / d1 {
        n1 * t * t
    } else if t < F::two() / d1 {
        let u = t - F::from_f32(1.5) / d1;
        n1 * u * u + F::from_f32(0.75)
    } else if t < F::from_f32(2.5) / d1 {
        let u = t - F::from_f32(2.25) / d1;
        n1 * u * u + F::from_f32(0.9375)
    } else {
        let u = t - F::from_f32(2.625) / d1;
        n1 * u * u + F::from_f32(0.984375)
    }
}

pub fn ease_in_out_bounce<F: Float>(t: F) -> F {
    if t < F::half() {
        (F::one() - ease_out_bounce(F::one() - F::two() * t)) / F::two()
    } else {
        (F::one() + ease_out_bounce(F::two() * t - F::one())) / F::two()
    }
}

pub fn cubic_bezier<F: Float>(t: F, x1: F, y1: F, x2: F, y2: F) -> F {
    if t <= F::zero() {
        return F::zero();
    }
    if t >= F::one() {
        return F::one();
    }

    let epsilon = F::from_f32(1e-7);
    let mut s = t;

    for _ in 0..8 {
        let bx = bezier_component(s, x1, x2);
        let dbx = bezier_derivative(s, x1, x2);
        if dbx.abs() < epsilon {
            break;
        }
        s = (s - (bx - t) / dbx).clamp(F::zero(), F::one());
    }

    let residual = (bezier_component(s, x1, x2) - t).abs();
    if residual > F::from_f32(1e-5) {
        let mut lo = F::zero();
        let mut hi = F::one();
        for _ in 0..20 {
            s = (lo + hi) / F::two();
            let bx = bezier_component(s, x1, x2);
            if bx < t {
                lo = s;
            } else {
                hi = s;
            }
        }
    }

    bezier_component(s, y1, y2)
}

fn bezier_component<F: Float>(s: F, p1: F, p2: F) -> F {
    let u = F::one() - s;
    let three = F::from_f32(3.0);
    three * u * u * s * p1 + three * u * s * s * p2 + s * s * s
}

fn bezier_derivative<F: Float>(s: F, p1: F, p2: F) -> F {
    let u = F::one() - s;
    let three = F::from_f32(3.0);
    let six = F::from_f32(6.0);
    three * u * u * p1 + six * u * s * (p2 - p1) + three * s * s * (F::one() - p2)
}

#[cfg(test)]
mod tests {
    use super::{
        cubic_bezier, ease_in_back, ease_in_out_quad, ease_in_quad, ease_out_bounce,
        ease_out_elastic, ease_out_quad, Easing,
    };

    const EPS: f32 = 1e-4;

    fn approx(a: f32, b: f32) -> bool {
        (a - b).abs() <= EPS
    }

    fn all_easings() -> [Easing<f32>; 32] {
        [
            Easing::Linear,
            Easing::EaseInQuad,
            Easing::EaseOutQuad,
            Easing::EaseInOutQuad,
            Easing::EaseInCubic,
            Easing::EaseOutCubic,
            Easing::EaseInOutCubic,
            Easing::EaseInQuart,
            Easing::EaseOutQuart,
            Easing::EaseInOutQuart,
            Easing::EaseInQuint,
            Easing::EaseOutQuint,
            Easing::EaseInOutQuint,
            Easing::EaseInSine,
            Easing::EaseOutSine,
            Easing::EaseInOutSine,
            Easing::EaseInExpo,
            Easing::EaseOutExpo,
            Easing::EaseInOutExpo,
            Easing::EaseInCirc,
            Easing::EaseOutCirc,
            Easing::EaseInOutCirc,
            Easing::EaseInBack,
            Easing::EaseOutBack,
            Easing::EaseInOutBack,
            Easing::EaseInElastic,
            Easing::EaseOutElastic,
            Easing::EaseInOutElastic,
            Easing::EaseInBounce,
            Easing::EaseOutBounce,
            Easing::EaseInOutBounce,
            Easing::CubicBezier {
                x1: 0.42,
                y1: 0.0,
                x2: 0.58,
                y2: 1.0,
            },
        ]
    }

    #[test]
    fn all_easings_at_zero() {
        for easing in all_easings() {
            assert!(approx(easing.evaluate(0.0), 0.0));
        }
    }

    #[test]
    fn all_easings_at_one() {
        for easing in all_easings() {
            assert!(approx(easing.evaluate(1.0), 1.0));
        }
    }

    #[test]
    fn linear_is_identity() {
        let e = Easing::Linear;
        for i in 0..=10 {
            let t = i as f32 / 10.0;
            assert!(approx(e.evaluate(t), t));
        }
    }

    #[test]
    fn quad_in_slow_start() {
        assert!(approx(ease_in_quad(0.5f32), 0.25));
    }

    #[test]
    fn quad_out_fast_start() {
        assert!(approx(ease_out_quad(0.5f32), 0.75));
    }

    #[test]
    fn cubic_bezier_linear() {
        let v = cubic_bezier(0.33f32, 0.0, 0.0, 1.0, 1.0);
        assert!(approx(v, 0.33));
    }

    #[test]
    fn cubic_bezier_css_ease() {
        let v = cubic_bezier(0.5f32, 0.25, 0.1, 0.25, 1.0);
        assert!(v > 0.75 && v < 0.9);
    }

    #[test]
    fn back_overshoots() {
        let v = ease_in_back(0.3f32);
        assert!(v < 0.0);
    }

    #[test]
    fn elastic_overshoots() {
        let mut saw = false;
        for i in 1..100 {
            let t = i as f32 / 100.0;
            if ease_out_elastic(t) > 1.0 {
                saw = true;
                break;
            }
        }
        assert!(saw);
    }

    #[test]
    fn bounce_never_negative() {
        for i in 0..=200 {
            let t = i as f32 / 200.0;
            assert!(ease_out_bounce(t) >= -EPS);
        }
    }

    #[test]
    fn in_out_symmetry() {
        assert!(approx(ease_in_out_quad(0.5f32), 0.5));
    }

    #[test]
    fn in_out_quad_midpoint() {
        assert!(approx(Easing::EaseInOutQuad.evaluate(0.5f32), 0.5));
    }
}
