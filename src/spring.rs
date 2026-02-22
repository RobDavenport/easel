use crate::float::Float;

/// Configuration for a spring-based tween.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SpringConfig<F: Float> {
    /// Spring stiffness (higher = faster / snappier).
    pub stiffness: F,
    /// Damping coefficient (higher = less oscillation).
    pub damping: F,
    /// Mass (higher = more inertia).
    pub mass: F,
    /// Threshold for considering the spring at rest.
    pub rest_threshold: F,
}

impl<F: Float> SpringConfig<F> {
    pub fn gentle() -> Self {
        Self {
            stiffness: F::from_f32(120.0),
            damping: F::from_f32(14.0),
            mass: F::one(),
            rest_threshold: F::from_f32(0.01),
        }
    }

    pub fn wobbly() -> Self {
        Self {
            stiffness: F::from_f32(180.0),
            damping: F::from_f32(12.0),
            mass: F::one(),
            rest_threshold: F::from_f32(0.01),
        }
    }

    pub fn stiff() -> Self {
        Self {
            stiffness: F::from_f32(400.0),
            damping: F::from_f32(28.0),
            mass: F::one(),
            rest_threshold: F::from_f32(0.01),
        }
    }

    pub fn slow() -> Self {
        Self {
            stiffness: F::from_f32(80.0),
            damping: F::from_f32(20.0),
            mass: F::one(),
            rest_threshold: F::from_f32(0.01),
        }
    }

    pub fn molasses() -> Self {
        Self {
            stiffness: F::from_f32(60.0),
            damping: F::from_f32(30.0),
            mass: F::from_f32(2.0),
            rest_threshold: F::from_f32(0.01),
        }
    }
}

/// Physics-based spring animation with retargetable target.
#[derive(Clone, Debug)]
pub struct SpringTween<F: Float> {
    value: F,
    velocity: F,
    target: F,
    config: SpringConfig<F>,
    at_rest: bool,
}

impl<F: Float> SpringTween<F> {
    pub fn new(initial: F, target: F, config: SpringConfig<F>) -> Self {
        Self {
            value: initial,
            velocity: F::zero(),
            target,
            config,
            at_rest: false,
        }
    }

    /// Advance by one tick and return current value.
    pub fn tick(&mut self) -> F {
        if self.at_rest {
            return self.value;
        }

        // One animation tick uses a fixed timestep to keep spring constants practical.
        let dt = F::from_f32(1.0 / 60.0);
        let displacement = self.value - self.target;
        let force = -self.config.stiffness * displacement - self.config.damping * self.velocity;
        let acceleration = force / self.config.mass;

        self.velocity = self.velocity + acceleration * dt;
        self.value = self.value + self.velocity * dt;

        let displacement_after = self.value - self.target;
        if self.velocity.abs() < self.config.rest_threshold
            && displacement_after.abs() < self.config.rest_threshold
        {
            self.value = self.target;
            self.velocity = F::zero();
            self.at_rest = true;
        }

        self.value
    }

    pub fn value(&self) -> F {
        self.value
    }

    pub fn velocity(&self) -> F {
        self.velocity
    }

    pub fn is_at_rest(&self) -> bool {
        self.at_rest
    }

    /// Change target mid-flight and wake if resting.
    pub fn set_target(&mut self, new_target: F) {
        self.target = new_target;
        self.at_rest = false;
    }

    /// Immediately set value and velocity.
    pub fn reset(&mut self, value: F, target: F) {
        self.value = value;
        self.velocity = F::zero();
        self.target = target;
        self.at_rest = false;
    }
}

#[cfg(test)]
mod tests {
    use crate::spring::{SpringConfig, SpringTween};

    const EPS: f32 = 0.05;

    #[test]
    fn spring_reaches_target() {
        let mut spring = SpringTween::new(0.0f32, 100.0, SpringConfig::gentle());
        for _ in 0..600 {
            spring.tick();
        }
        assert!((spring.value() - 100.0).abs() < EPS);
    }

    #[test]
    fn spring_wobbly_overshoots() {
        let mut spring = SpringTween::new(0.0f32, 100.0, SpringConfig::wobbly());
        let mut overshot = false;
        for _ in 0..300 {
            if spring.tick() > 100.0 {
                overshot = true;
                break;
            }
        }
        assert!(overshot);
    }

    #[test]
    fn spring_stiff_fast() {
        fn settle_ticks(config: SpringConfig<f32>) -> usize {
            let mut spring = SpringTween::new(0.0f32, 100.0, config);
            for i in 1..=1000 {
                spring.tick();
                if spring.is_at_rest() {
                    return i;
                }
            }
            1000
        }

        let stiff_ticks = settle_ticks(SpringConfig::stiff());
        let gentle_ticks = settle_ticks(SpringConfig::gentle());
        assert!(stiff_ticks < gentle_ticks);
    }

    #[test]
    fn spring_at_rest() {
        let mut spring = SpringTween::new(0.0f32, 10.0, SpringConfig::gentle());
        for _ in 0..600 {
            spring.tick();
            if spring.is_at_rest() {
                break;
            }
        }
        assert!(spring.is_at_rest());
    }

    #[test]
    fn spring_retarget() {
        let mut spring = SpringTween::new(0.0f32, 100.0, SpringConfig::gentle());
        for _ in 0..60 {
            spring.tick();
        }
        let before = spring.value();
        spring.set_target(200.0);
        let after = spring.tick();
        assert!(after > before);
    }

    #[test]
    fn spring_retarget_wakes() {
        let mut spring = SpringTween::new(0.0f32, 10.0, SpringConfig::gentle());
        for _ in 0..1000 {
            spring.tick();
            if spring.is_at_rest() {
                break;
            }
        }
        assert!(spring.is_at_rest());
        spring.set_target(20.0);
        assert!(!spring.is_at_rest());
    }

    #[test]
    fn spring_deterministic() {
        let mut a = SpringTween::new(0.0f32, 100.0, SpringConfig::stiff());
        let mut b = SpringTween::new(0.0f32, 100.0, SpringConfig::stiff());
        for _ in 0..200 {
            let va = a.tick();
            let vb = b.tick();
            assert!((va - vb).abs() < 1e-6);
        }
    }
}
