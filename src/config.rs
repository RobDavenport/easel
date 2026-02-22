use crate::easing::Easing;
use crate::float::Float;

/// CSS-like easing presets.
pub struct TweenConfig;

impl TweenConfig {
    /// CSS `ease`: cubic-bezier(0.25, 0.1, 0.25, 1.0)
    pub fn ease<F: Float>() -> Easing<F> {
        Easing::CubicBezier {
            x1: F::from_f32(0.25),
            y1: F::from_f32(0.1),
            x2: F::from_f32(0.25),
            y2: F::one(),
        }
    }

    /// CSS `ease-in`: cubic-bezier(0.42, 0, 1, 1)
    pub fn ease_in<F: Float>() -> Easing<F> {
        Easing::CubicBezier {
            x1: F::from_f32(0.42),
            y1: F::zero(),
            x2: F::one(),
            y2: F::one(),
        }
    }

    /// CSS `ease-out`: cubic-bezier(0, 0, 0.58, 1)
    pub fn ease_out<F: Float>() -> Easing<F> {
        Easing::CubicBezier {
            x1: F::zero(),
            y1: F::zero(),
            x2: F::from_f32(0.58),
            y2: F::one(),
        }
    }

    /// CSS `ease-in-out`: cubic-bezier(0.42, 0, 0.58, 1)
    pub fn ease_in_out<F: Float>() -> Easing<F> {
        Easing::CubicBezier {
            x1: F::from_f32(0.42),
            y1: F::zero(),
            x2: F::from_f32(0.58),
            y2: F::one(),
        }
    }

    /// CSS `linear`.
    pub fn linear<F: Float>() -> Easing<F> {
        Easing::Linear
    }

    /// Step-like snap to target.
    pub fn snap<F: Float>() -> Easing<F> {
        Easing::CubicBezier {
            x1: F::zero(),
            y1: F::one(),
            x2: F::zero(),
            y2: F::one(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::TweenConfig;
    use crate::easing::Easing;

    #[test]
    fn config_ease_matches_css() {
        let easing = TweenConfig::ease::<f32>();
        match easing {
            Easing::CubicBezier { x1, y1, x2, y2 } => {
                assert!((x1 - 0.25).abs() < 1e-6);
                assert!((y1 - 0.1).abs() < 1e-6);
                assert!((x2 - 0.25).abs() < 1e-6);
                assert!((y2 - 1.0).abs() < 1e-6);
            }
            _ => panic!("expected cubic bezier"),
        }
    }

    #[test]
    fn config_snap() {
        let easing = TweenConfig::snap::<f32>();
        assert!((easing.evaluate(0.0) - 0.0).abs() < 1e-6);
        assert!(easing.evaluate(0.1) > 0.1);
        assert!(easing.evaluate(0.9) > 0.95);
        assert!((easing.evaluate(1.0) - 1.0).abs() < 1e-6);
    }
}
