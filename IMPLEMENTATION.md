# easel Implementation Plan

## What This Is

A `no_std` Rust library for game-tick-friendly animation primitives. Provides tweening, all 30 Penner easing functions, multi-point keyframes, spring physics, and timeline composition — all using integer tick timing for deterministic, rollback-safe animation.

**Key design principles:**
- All timing in `u32` ticks — no floats for time, fully deterministic and rollback-safe
- `Lerp<F>` trait for user-extensible interpolatable types (colors, angles, vectors, etc.)
- `Easing` enum for data-driven selection + free functions for zero-overhead direct use
- Sequence / Parallel / Stagger are homogeneous (generic over `T`)
- `Timeline` is a heterogeneous clock — it doesn't know about tween types, just timing
- `SpringTween` uses semi-implicit Euler (simpler than softy's analytical, retargetable mid-flight)
- No randomness → no `rand_core` dependency. Only dependency is `libm`

**Reference implementations:** See sibling repo `softy/` for analytical springs (more precise but not retargetable). easel's springs are simpler, designed for UI/animation use cases where retargetability matters more than physical accuracy.

## Hard Rules

- `#![no_std]` with `extern crate alloc` — no std dependency in core library
- `libm` is the ONLY dependency (no `rand_core` — tweening has no randomness)
- All tests: `cargo test --target x86_64-pc-windows-msvc`
- WASM check: `cargo build --target wasm32-unknown-unknown --release`
- Deterministic: same tween + same tick count = same output value
- All timing uses `u32` ticks — never float-based seconds
- All math uses the `Float` trait — never raw `f32`/`f64` directly in generic code
- Easing functions must satisfy: `f(0) = 0`, `f(1) = 1` (except elastic/bounce which overshoot)

---

## Phase 1: Float + Lerp

### Step 1: Float Trait (float.rs)

Same pattern as sibling libraries (softy, navex, cogwise). Your job: implement using `libm` for `no_std` math.

```rust
/// Trait abstracting floating-point operations for animation math.
///
/// Implemented for `f32` and `f64`. Uses `libm` for no_std transcendental functions.
pub trait Float:
    Copy + Clone + PartialEq + PartialOrd
    + core::ops::Add<Output = Self>
    + core::ops::Sub<Output = Self>
    + core::ops::Mul<Output = Self>
    + core::ops::Div<Output = Self>
    + core::ops::Neg<Output = Self>
    + Default + core::fmt::Debug
{
    fn zero() -> Self;
    fn one() -> Self;
    fn half() -> Self;
    fn two() -> Self;
    fn pi() -> Self;
    fn tau() -> Self;  // 2π
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

    /// Remap value from [0, 1] to [0, 1] using `t` as progress.
    fn remap01(self, from_min: Self, from_max: Self) -> Self {
        (self - from_min) / (from_max - from_min)
    }
}

// Implement for f32 and f64 using libm::{sinf, cosf, sqrtf, powf, expf, floorf}
```

**IMPORTANT:** Since this is `no_std`, you CANNOT use `f32::sin()` etc. directly (those are in std). You must use the `libm` crate for math functions:

```toml
[dependencies]
libm = "0.2"
```

### Step 2: Lerp Trait (lerp.rs)

```rust
/// Trait for types that can be linearly interpolated.
///
/// `t = 0.0` → returns `self`, `t = 1.0` → returns `other`.
///
/// Built-in implementations cover scalars, tuples (2D/3D/4D),
/// arrays, colors, and angles.
///
/// # Example
/// ```
/// use easel::Lerp;
///
/// let a = 0.0f32;
/// let b = 10.0f32;
/// assert_eq!(a.lerp(&b, 0.5), 5.0);
/// ```
pub trait Lerp<F: Float> {
    /// Linearly interpolate between `self` and `other` by factor `t`.
    fn lerp(&self, other: &Self, t: F) -> Self;
}

// === Built-in implementations ===

/// Scalar float: standard linear interpolation.
impl<F: Float> Lerp<F> for F {
    fn lerp(&self, other: &Self, t: F) -> Self {
        *self + (*other - *self) * t
    }
}

/// 2D tuple: component-wise lerp.
impl<F: Float> Lerp<F> for (F, F) {
    fn lerp(&self, other: &Self, t: F) -> Self {
        (self.0.lerp(other.0, t), self.1.lerp(other.1, t))
    }
}

/// 3D tuple: component-wise lerp.
impl<F: Float> Lerp<F> for (F, F, F) {
    fn lerp(&self, other: &Self, t: F) -> Self {
        (
            self.0.lerp(other.0, t),
            self.1.lerp(other.1, t),
            self.2.lerp(other.2, t),
        )
    }
}

/// 4D tuple: component-wise lerp.
impl<F: Float> Lerp<F> for (F, F, F, F) {
    fn lerp(&self, other: &Self, t: F) -> Self {
        (
            self.0.lerp(other.0, t),
            self.1.lerp(other.1, t),
            self.2.lerp(other.2, t),
            self.3.lerp(other.3, t),
        )
    }
}

/// Fixed-size array: component-wise lerp.
impl<F: Float, const N: usize> Lerp<F> for [F; N] {
    fn lerp(&self, other: &Self, t: F) -> Self {
        let mut result = *self;
        for i in 0..N {
            result[i] = self[i].lerp(other[i], t);
        }
        result
    }
}

/// RGBA color with premultiplied alpha interpolation.
///
/// Premultiplied alpha lerp prevents dark fringes when blending
/// between transparent and opaque colors.
///
/// ```text
/// Premultiplied: (r*a, g*a, b*a, a) lerps component-wise
/// Then unpremultiply: (r/a, g/a, b/a, a)
/// ```
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Rgba<F: Float> {
    pub r: F,
    pub g: F,
    pub b: F,
    pub a: F,
}

impl<F: Float> Rgba<F> {
    pub fn new(r: F, g: F, b: F, a: F) -> Self { Rgba { r, g, b, a } }
}

impl<F: Float> Lerp<F> for Rgba<F> {
    fn lerp(&self, other: &Self, t: F) -> Self {
        // Premultiply
        let pre_self = (self.r * self.a, self.g * self.a, self.b * self.a, self.a);
        let pre_other = (other.r * other.a, other.g * other.a, other.b * other.a, other.a);

        // Lerp in premultiplied space
        let a = pre_self.3.lerp(pre_other.3, t);
        if a <= F::zero() {
            return Rgba::new(F::zero(), F::zero(), F::zero(), F::zero());
        }

        let r = pre_self.0.lerp(pre_other.0, t) / a;
        let g = pre_self.1.lerp(pre_other.1, t) / a;
        let b = pre_self.2.lerp(pre_other.2, t) / a;

        Rgba::new(r, g, b, a)
    }
}

/// Angle in radians with shortest-path interpolation.
///
/// Handles the wraparound case: lerping from 350° to 10° goes through
/// 0° (20° arc), not through 180° (340° arc).
///
/// ```text
/// Algorithm:
///     diff = to - from
///     // Normalize diff to [-π, π]
///     while diff > π:  diff -= 2π
///     while diff < -π: diff += 2π
///     result = from + diff * t
/// ```
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Angle<F: Float> {
    pub radians: F,
}

impl<F: Float> Angle<F> {
    pub fn from_radians(r: F) -> Self { Angle { radians: r } }
    pub fn from_degrees(d: F) -> Self {
        Angle { radians: d * F::pi() / F::from_f32(180.0) }
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

        // Normalize to [-π, π]
        while diff > pi { diff = diff - tau; }
        while diff < -pi { diff = diff + tau; }

        Angle { radians: self.radians + diff * t }
    }
}
```

**Tests for Phase 1:**
- `float_f32_basics` — zero, one, half, from_f32 round-trip
- `float_f32_math` — sqrt(4)=2, sin(0)=0, cos(0)=1
- `lerp_scalar_midpoint` — 0.0.lerp(10.0, 0.5) = 5.0
- `lerp_scalar_endpoints` — t=0 returns start, t=1 returns end
- `lerp_tuple2` — component-wise (0,0).lerp((10,20), 0.5) = (5,10)
- `lerp_tuple3` — 3D component-wise
- `lerp_array` — [0;4].lerp([4;4], 0.25) = [1;4]
- `lerp_rgba_opaque` — standard lerp when both fully opaque
- `lerp_rgba_transparent` — premultiplied alpha prevents dark fringes
- `lerp_angle_short_path` — 350°→10° goes through 0° (20° arc)
- `lerp_angle_long_way` — 10°→350° goes through 0° backward (20° arc)
- `lerp_angle_half` — 0°→180° at t=0.5 = 90°

---

## Phase 2: Easing Functions

### Step 3: Easing Enum (easing.rs)

```rust
/// All 30 standard Penner easing functions plus cubic Bezier.
///
/// Each function maps a normalized time `t ∈ [0, 1]` to an output
/// value. Most outputs are also in [0, 1], but elastic, bounce, and
/// back easings can temporarily go outside this range.
///
/// # Data-driven usage
/// Use the enum when easing type needs to be stored/serialized:
/// ```
/// use easel::Easing;
/// let e = Easing::EaseInOutCubic;
/// let value: f32 = e.evaluate(0.5);
/// ```
///
/// # Direct usage
/// Use free functions for zero-overhead when the easing is known at compile time:
/// ```
/// use easel::easing;
/// let value = easing::ease_in_cubic(0.5f32);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum Easing<F: Float> {
    Linear,

    // Polynomial
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

    // Trigonometric
    EaseInSine,
    EaseOutSine,
    EaseInOutSine,

    // Exponential
    EaseInExpo,
    EaseOutExpo,
    EaseInOutExpo,

    // Circular
    EaseInCirc,
    EaseOutCirc,
    EaseInOutCirc,

    // Overshoot
    EaseInBack,
    EaseOutBack,
    EaseInOutBack,

    // Elastic
    EaseInElastic,
    EaseOutElastic,
    EaseInOutElastic,

    // Bounce
    EaseInBounce,
    EaseOutBounce,
    EaseInOutBounce,

    /// Cubic Bezier with custom control points.
    /// Models CSS `cubic-bezier(x1, y1, x2, y2)`.
    CubicBezier { x1: F, y1: F, x2: F, y2: F },
}

impl<F: Float> Easing<F> {
    /// Evaluate the easing function at time `t` (should be in [0, 1]).
    pub fn evaluate(&self, t: F) -> F {
        match self {
            Easing::Linear => t,
            Easing::EaseInQuad => ease_in_quad(t),
            Easing::EaseOutQuad => ease_out_quad(t),
            // ... all 30 + CubicBezier
            Easing::CubicBezier { x1, y1, x2, y2 } => {
                cubic_bezier(t, *x1, *y1, *x2, *y2)
            }
        }
    }
}
```

### Step 4: Free Functions (easing.rs)

Your job: implement ALL of these. The formulas are in the Algorithm References section below.

```rust
// === Polynomial ===

/// `t²`
pub fn ease_in_quad<F: Float>(t: F) -> F { t * t }

/// `1 - (1-t)²`
pub fn ease_out_quad<F: Float>(t: F) -> F {
    let u = F::one() - t;
    F::one() - u * u
}

/// `t < 0.5 ? 2t² : 1 - (-2t+2)²/2`
pub fn ease_in_out_quad<F: Float>(t: F) -> F {
    if t < F::half() {
        F::two() * t * t
    } else {
        let u = F::from_f32(-2.0) * t + F::two();
        F::one() - u * u / F::two()
    }
}

/// `t³`
pub fn ease_in_cubic<F: Float>(t: F) -> F { t * t * t }

/// `1 - (1-t)³`
pub fn ease_out_cubic<F: Float>(t: F) -> F {
    let u = F::one() - t;
    F::one() - u * u * u
}

/// `t < 0.5 ? 4t³ : 1 - (-2t+2)³/2`
pub fn ease_in_out_cubic<F: Float>(t: F) -> F { /* ... */ }

pub fn ease_in_quart<F: Float>(t: F) -> F { t * t * t * t }
pub fn ease_out_quart<F: Float>(t: F) -> F { /* ... */ }
pub fn ease_in_out_quart<F: Float>(t: F) -> F { /* ... */ }

pub fn ease_in_quint<F: Float>(t: F) -> F { t * t * t * t * t }
pub fn ease_out_quint<F: Float>(t: F) -> F { /* ... */ }
pub fn ease_in_out_quint<F: Float>(t: F) -> F { /* ... */ }

// === Trigonometric ===

/// `1 - cos(t × π/2)`
pub fn ease_in_sine<F: Float>(t: F) -> F {
    F::one() - (t * F::pi() / F::two()).cos()
}

/// `sin(t × π/2)`
pub fn ease_out_sine<F: Float>(t: F) -> F {
    (t * F::pi() / F::two()).sin()
}

/// `-(cos(πt) - 1) / 2`
pub fn ease_in_out_sine<F: Float>(t: F) -> F {
    -((F::pi() * t).cos() - F::one()) / F::two()
}

// === Exponential ===

/// `t == 0 ? 0 : 2^(10t - 10)`
pub fn ease_in_expo<F: Float>(t: F) -> F {
    if t == F::zero() { F::zero() }
    else { F::two().powf(F::from_f32(10.0) * t - F::from_f32(10.0)) }
}

/// `t == 1 ? 1 : 1 - 2^(-10t)`
pub fn ease_out_expo<F: Float>(t: F) -> F {
    if t == F::one() { F::one() }
    else { F::one() - F::two().powf(F::from_f32(-10.0) * t) }
}

pub fn ease_in_out_expo<F: Float>(t: F) -> F { /* ... */ }

// === Circular ===

/// `1 - sqrt(1 - t²)`
pub fn ease_in_circ<F: Float>(t: F) -> F {
    F::one() - (F::one() - t * t).sqrt()
}

/// `sqrt(1 - (t-1)²)`
pub fn ease_out_circ<F: Float>(t: F) -> F {
    let u = t - F::one();
    (F::one() - u * u).sqrt()
}

pub fn ease_in_out_circ<F: Float>(t: F) -> F { /* ... */ }

// === Back (overshoot) ===

/// `c3×t³ - c1×t²` where c1 = 1.70158, c3 = c1 + 1
pub fn ease_in_back<F: Float>(t: F) -> F {
    let c1 = F::from_f32(1.70158);
    let c3 = c1 + F::one();
    c3 * t * t * t - c1 * t * t
}

/// `1 + c3×(t-1)³ + c1×(t-1)²`
pub fn ease_out_back<F: Float>(t: F) -> F {
    let c1 = F::from_f32(1.70158);
    let c3 = c1 + F::one();
    let u = t - F::one();
    F::one() + c3 * u * u * u + c1 * u * u
}

pub fn ease_in_out_back<F: Float>(t: F) -> F { /* ... */ }

// === Elastic ===

/// `-2^(10t-10) × sin((10t - 10.75) × 2π/3)`
pub fn ease_in_elastic<F: Float>(t: F) -> F {
    if t == F::zero() || t == F::one() { return t; }
    let c4 = F::tau() / F::from_f32(3.0);
    -(F::two().powf(F::from_f32(10.0) * t - F::from_f32(10.0)))
        * ((F::from_f32(10.0) * t - F::from_f32(10.75)) * c4).sin()
}

/// `2^(-10t) × sin((10t - 0.75) × 2π/3) + 1`
pub fn ease_out_elastic<F: Float>(t: F) -> F {
    if t == F::zero() || t == F::one() { return t; }
    let c4 = F::tau() / F::from_f32(3.0);
    F::two().powf(F::from_f32(-10.0) * t)
        * ((F::from_f32(10.0) * t - F::from_f32(0.75)) * c4).sin()
        + F::one()
}

pub fn ease_in_out_elastic<F: Float>(t: F) -> F { /* ... */ }

// === Bounce ===

/// `1 - ease_out_bounce(1 - t)`
pub fn ease_in_bounce<F: Float>(t: F) -> F {
    F::one() - ease_out_bounce(F::one() - t)
}

/// Piecewise quadratic (4 segments)
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

pub fn ease_in_out_bounce<F: Float>(t: F) -> F { /* ... */ }
```

### Step 5: Cubic Bezier (easing.rs)

```rust
/// Evaluate a cubic Bezier easing curve.
///
/// Given control points (x1, y1) and (x2, y2) (the two middle control points;
/// the first is always (0,0) and the last is always (1,1)):
///
/// ```text
/// B_x(s) = 3(1-s)²s·x1 + 3(1-s)s²·x2 + s³
/// B_y(s) = 3(1-s)²s·y1 + 3(1-s)s²·y2 + s³
///
/// To find y for a given x (our input t):
///     1. Solve B_x(s) = t for s using Newton-Raphson
///     2. Return B_y(s)
///
/// Newton-Raphson iteration:
///     s₀ = t  (initial guess)
///     For 8 iterations:
///         sₙ₊₁ = sₙ - (B_x(sₙ) - t) / B_x'(sₙ)
///     Where B_x'(s) = 3(1-s)²·x1 + 6(1-s)s·(x2-x1) + 3s²·(1-x2)
///     If |B_x'(sₙ)| < ε: fall back to bisection
/// ```
pub fn cubic_bezier<F: Float>(t: F, x1: F, y1: F, x2: F, y2: F) -> F {
    if t <= F::zero() { return F::zero(); }
    if t >= F::one() { return F::one(); }

    // Find s such that B_x(s) = t
    let mut s = t; // Initial guess
    for _ in 0..8 {
        let bx = bezier_component(s, x1, x2);
        let dbx = bezier_derivative(s, x1, x2);
        if dbx.abs() < F::from_f32(1e-7) { break; } // avoid division by zero
        s = s - (bx - t) / dbx;
        s = s.clamp(F::zero(), F::one());
    }

    // Return B_y(s)
    bezier_component(s, y1, y2)
}

/// Evaluate one component of the cubic Bezier: 3(1-s)²s·p1 + 3(1-s)s²·p2 + s³
fn bezier_component<F: Float>(s: F, p1: F, p2: F) -> F {
    let u = F::one() - s;
    let three = F::from_f32(3.0);
    three * u * u * s * p1 + three * u * s * s * p2 + s * s * s
}

/// Derivative of Bezier component: 3(1-s)²·p1 + 6(1-s)s·(p2-p1) + 3s²·(1-p2)
fn bezier_derivative<F: Float>(s: F, p1: F, p2: F) -> F {
    let u = F::one() - s;
    let three = F::from_f32(3.0);
    let six = F::from_f32(6.0);
    three * u * u * p1 + six * u * s * (p2 - p1) + three * s * s * (F::one() - p2)
}
```

**Tests for Phase 2:**
- `all_easings_at_zero` — every easing evaluates to 0.0 at t=0
- `all_easings_at_one` — every easing evaluates to 1.0 at t=1
- `linear_is_identity` — linear(t) = t for all t
- `quad_in_slow_start` — ease_in_quad(0.5) = 0.25 (below midpoint)
- `quad_out_fast_start` — ease_out_quad(0.5) = 0.75 (above midpoint)
- `cubic_bezier_linear` — CubicBezier(0.0, 0.0, 1.0, 1.0) ≈ linear
- `cubic_bezier_css_ease` — CubicBezier(0.25, 0.1, 0.25, 1.0) matches CSS ease
- `back_overshoots` — ease_in_back goes below 0 for some t
- `elastic_overshoots` — ease_out_elastic exceeds 1 for some t
- `bounce_never_negative` — ease_out_bounce ≥ 0 for all t in [0,1]
- `in_out_symmetry` — ease_in_out_quad(0.5) = 0.5

---

## Phase 3: Core Tween

### Step 6: State + Loop Mode (state.rs, loop_mode.rs)

```rust
/// Current state of a tween or animation.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum TweenState {
    /// Not yet started.
    #[default]
    Idle,
    /// Currently animating.
    Playing,
    /// Paused mid-animation.
    Paused,
    /// Animation has completed.
    Finished,
}

/// How a tween behaves when it reaches the end.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum LoopMode {
    /// Play once and finish.
    #[default]
    Once,
    /// Play N times and finish.
    Count(u32),
    /// Play forever.
    Infinite,
    /// Play forward, then backward, then finish.
    PingPong,
    /// PingPong N times (each direction counts as half).
    PingPongCount(u32),
}

/// Current playback direction.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum PlayDirection {
    #[default]
    Forward,
    Backward,
}
```

### Step 7: TweenId (tween.rs)

```rust
/// Opaque identifier for a tween in a Timeline.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TweenId(pub u32);
```

### Step 8: Tween (tween.rs)

```rust
/// A single from→to animation with easing, delay, and looping.
///
/// All timing is in integer ticks for determinism.
///
/// # Example
/// ```
/// use easel::{Tween, Easing};
///
/// // Animate position from 0 to 100 over 60 ticks with ease-out
/// let mut tween = Tween::new(0.0f32, 100.0, 60)
///     .with_easing(Easing::EaseOutCubic)
///     .with_delay(10);
///
/// // Tick 60 + 10 delay = 70 times
/// for _ in 0..70 {
///     let value = tween.tick();
///     println!("position: {value}");
/// }
/// assert!(tween.is_finished());
/// ```
#[derive(Clone, Debug)]
pub struct Tween<T: Lerp<F>, F: Float> {
    from: T,
    to: T,
    duration: u32,
    elapsed: u32,
    easing: Easing<F>,
    state: TweenState,
    loop_mode: LoopMode,
    delay: u32,
    delay_remaining: u32,
    loops_completed: u32,
    direction: PlayDirection,
}

impl<T: Lerp<F> + Clone, F: Float> Tween<T, F> {
    /// Create a tween from `from` to `to` over `duration` ticks.
    /// Starts in Playing state with linear easing.
    pub fn new(from: T, to: T, duration: u32) -> Self {
        Tween {
            from,
            to,
            duration,
            elapsed: 0,
            easing: Easing::Linear,
            state: TweenState::Playing,
            loop_mode: LoopMode::Once,
            delay: 0,
            delay_remaining: 0,
            loops_completed: 0,
            direction: PlayDirection::Forward,
        }
    }

    /// Set the easing function (builder pattern).
    pub fn with_easing(mut self, easing: Easing<F>) -> Self {
        self.easing = easing; self
    }

    /// Set loop mode.
    pub fn with_loop(mut self, mode: LoopMode) -> Self {
        self.loop_mode = mode; self
    }

    /// Set delay in ticks before the animation starts.
    pub fn with_delay(mut self, ticks: u32) -> Self {
        self.delay = ticks;
        self.delay_remaining = ticks;
        self
    }

    /// Advance by one tick and return the current interpolated value.
    ///
    /// Algorithm:
    /// ```text
    /// if state != Playing: return current value
    /// if delay_remaining > 0: delay_remaining -= 1; return from
    /// elapsed += 1
    /// raw_t = elapsed / duration  (as float)
    /// if direction == Backward: raw_t = 1 - raw_t
    /// eased_t = easing.evaluate(raw_t)
    /// value = from.lerp(to, eased_t)
    /// if elapsed >= duration:
    ///     handle loop (reset elapsed, flip direction, increment count, or finish)
    /// return value
    /// ```
    pub fn tick(&mut self) -> T { /* ... */ }

    /// Get the current value without advancing.
    pub fn value(&self) -> T { /* ... */ }

    /// Normalized progress [0, 1] within the current loop iteration.
    pub fn progress(&self) -> F { /* ... */ }

    /// Whether the tween has completed all iterations.
    pub fn is_finished(&self) -> bool { self.state == TweenState::Finished }

    /// Current state.
    pub fn state(&self) -> TweenState { self.state }

    /// Reset to initial state.
    pub fn reset(&mut self) { /* ... */ }

    /// Pause the animation.
    pub fn pause(&mut self) { /* ... */ }

    /// Resume a paused animation.
    pub fn resume(&mut self) { /* ... */ }

    /// Change the target value mid-flight (retargeting).
    /// Does NOT reset elapsed — the tween smoothly redirects.
    pub fn set_target(&mut self, new_to: T) {
        self.to = new_to;
    }

    /// Change both from and to mid-flight (useful for chaining).
    pub fn set_range(&mut self, new_from: T, new_to: T) {
        self.from = new_from;
        self.to = new_to;
    }

    /// Total duration including delay.
    pub fn total_duration(&self) -> u32 { self.delay + self.duration }

    /// Loops completed so far.
    pub fn loops_completed(&self) -> u32 { self.loops_completed }
}
```

**Tests for Phase 3:**
- `tween_basic_linear` — 0→100 over 10 ticks: tick 5 = 50
- `tween_easing_applied` — ease_in_quad at midpoint: value < 50
- `tween_delay` — value stays at `from` during delay ticks
- `tween_finishes` — state is Finished after duration
- `tween_loop_count` — Count(3) plays 3 times then finishes
- `tween_loop_infinite` — still Playing after 1000 ticks
- `tween_pingpong` — goes forward then backward
- `tween_pingpong_midpoint` — at turnaround point, value = to
- `tween_pause_resume` — value doesn't change while paused
- `tween_reset` — returns to from value and Playing state
- `tween_retarget` — set_target changes endpoint smoothly
- `tween_progress` — progress() returns 0.5 at midpoint

---

## Phase 4: Keyframes

### Step 9: Keyframe + Keyframes (keyframes.rs)

```rust
/// A single point in a keyframed animation.
///
/// Each keyframe specifies a value at a specific tick, and the easing
/// function to use when interpolating TO the next keyframe.
#[derive(Clone, Debug)]
pub struct Keyframe<T: Lerp<F>, F: Float> {
    /// The value at this keyframe.
    pub value: T,
    /// Tick position (absolute, from start of animation).
    pub tick: u32,
    /// Easing function applied when interpolating from this keyframe to the next.
    pub easing: Easing<F>,
}

/// Multi-point animation with per-segment easing.
///
/// Keyframes must be in tick order. The animation interpolates between
/// adjacent keyframes using each segment's easing function.
///
/// # Example
/// ```
/// use easel::{Keyframes, Keyframe, Easing};
///
/// let mut kf = Keyframes::new(vec![
///     Keyframe { value: 0.0f32, tick: 0, easing: Easing::Linear },
///     Keyframe { value: 100.0, tick: 30, easing: Easing::EaseOutCubic },
///     Keyframe { value: 50.0, tick: 60, easing: Easing::EaseInOutQuad },
/// ]);
///
/// // Tick through the animation
/// for _ in 0..60 {
///     let value = kf.tick();
/// }
/// ```
#[derive(Clone, Debug)]
pub struct Keyframes<T: Lerp<F>, F: Float> {
    frames: Vec<Keyframe<T, F>>,
    elapsed: u32,
    state: TweenState,
    loop_mode: LoopMode,
    loops_completed: u32,
}

impl<T: Lerp<F> + Clone, F: Float> Keyframes<T, F> {
    pub fn new(frames: Vec<Keyframe<T, F>>) -> Self { /* ... */ }

    pub fn with_loop(mut self, mode: LoopMode) -> Self { /* ... */ }

    /// Advance by one tick and return the interpolated value.
    ///
    /// Algorithm:
    /// ```text
    /// 1. Binary search for the segment: find i where
    ///    frames[i].tick <= elapsed < frames[i+1].tick
    /// 2. Compute local t within the segment:
    ///    segment_duration = frames[i+1].tick - frames[i].tick
    ///    local_elapsed = elapsed - frames[i].tick
    ///    raw_t = local_elapsed / segment_duration
    /// 3. Apply the segment's easing:
    ///    eased_t = frames[i].easing.evaluate(raw_t)
    /// 4. Interpolate:
    ///    value = frames[i].value.lerp(frames[i+1].value, eased_t)
    /// ```
    pub fn tick(&mut self) -> T { /* ... */ }

    pub fn value(&self) -> T { /* ... */ }

    pub fn total_duration(&self) -> u32 {
        self.frames.last().map(|f| f.tick).unwrap_or(0)
    }

    pub fn progress(&self) -> F { /* ... */ }

    pub fn is_finished(&self) -> bool { self.state == TweenState::Finished }

    pub fn reset(&mut self) { /* ... */ }
}
```

**Tests for Phase 4:**
- `keyframes_two_point` — equivalent to basic Tween
- `keyframes_three_point` — hits intermediate value at keyframe tick
- `keyframes_different_easings` — segment 1 uses easing A, segment 2 uses easing B
- `keyframes_at_keyframe_tick` — at exact keyframe tick, returns keyframe value
- `keyframes_total_duration` — matches last keyframe tick
- `keyframes_loop` — replays after reaching end

---

## Phase 5: Spring Physics

### Step 10: SpringConfig (spring.rs)

```rust
/// Configuration for a spring-based tween.
///
/// Springs provide physically-motivated animation with overshoot
/// and natural settling. They are retargetable mid-flight (just
/// change the target and the spring adjusts smoothly).
///
/// # Presets
/// ```
/// use easel::SpringConfig;
///
/// let gentle = SpringConfig::gentle();   // Slow, no overshoot
/// let wobbly = SpringConfig::wobbly();   // Fast, lots of bounce
/// let stiff = SpringConfig::stiff();     // Fast, minimal overshoot
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SpringConfig<F: Float> {
    /// Spring stiffness (higher = faster / snappier).
    /// Typical range: 50 - 500.
    pub stiffness: F,
    /// Damping coefficient (higher = less oscillation).
    /// Typical range: 5 - 40.
    pub damping: F,
    /// Mass (higher = more inertia, slower response).
    /// Typical range: 0.5 - 5.
    pub mass: F,
    /// When |velocity| and |displacement| are below this, the spring is at rest.
    pub rest_threshold: F,
}

impl<F: Float> SpringConfig<F> {
    /// Gentle: slow approach, no overshoot. Good for background transitions.
    pub fn gentle() -> Self {
        SpringConfig {
            stiffness: F::from_f32(120.0),
            damping: F::from_f32(14.0),
            mass: F::one(),
            rest_threshold: F::from_f32(0.01),
        }
    }

    /// Wobbly: fast with lots of bounce. Good for playful UI elements.
    pub fn wobbly() -> Self {
        SpringConfig {
            stiffness: F::from_f32(180.0),
            damping: F::from_f32(12.0),
            mass: F::one(),
            rest_threshold: F::from_f32(0.01),
        }
    }

    /// Stiff: very fast, slight overshoot. Good for snappy interactions.
    pub fn stiff() -> Self {
        SpringConfig {
            stiffness: F::from_f32(400.0),
            damping: F::from_f32(28.0),
            mass: F::one(),
            rest_threshold: F::from_f32(0.01),
        }
    }

    /// Slow: overdamped, very gradual approach. Good for ambient animations.
    pub fn slow() -> Self {
        SpringConfig {
            stiffness: F::from_f32(80.0),
            damping: F::from_f32(20.0),
            mass: F::one(),
            rest_threshold: F::from_f32(0.01),
        }
    }

    /// Molasses: extremely slow and viscous. Good for drag-like effects.
    pub fn molasses() -> Self {
        SpringConfig {
            stiffness: F::from_f32(60.0),
            damping: F::from_f32(30.0),
            mass: F::from_f32(2.0),
            rest_threshold: F::from_f32(0.01),
        }
    }
}
```

### Step 11: SpringTween (spring.rs)

```rust
/// Physics-based spring animation with retargetable target.
///
/// Uses semi-implicit Euler integration (simpler than softy's analytical
/// springs, but naturally handles retargeting mid-flight).
///
/// # Physics
/// ```text
/// force = -stiffness * (value - target) - damping * velocity
/// acceleration = force / mass
/// velocity += acceleration     // dt = 1 tick (semi-implicit: update velocity first)
/// value += velocity            // then update position with new velocity
///
/// Rest check:
///     if |velocity| < threshold AND |value - target| < threshold:
///         value = target
///         velocity = 0
///         at_rest = true
/// ```
///
/// # Why semi-implicit Euler?
/// Standard Euler updates position first, then velocity — this can
/// add energy and cause instability. Semi-implicit Euler updates
/// velocity first, then uses the NEW velocity for position — this
/// is always stable and slightly damped, which is desirable for
/// animation springs.
///
/// # Example
/// ```
/// use easel::{SpringTween, SpringConfig};
///
/// let mut spring = SpringTween::new(0.0f32, 100.0, SpringConfig::wobbly());
///
/// for _ in 0..120 {
///     let value = spring.tick();
///     if spring.is_at_rest() { break; }
/// }
///
/// // Retarget mid-flight
/// spring.set_target(200.0);
/// ```
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
        SpringTween {
            value: initial,
            velocity: F::zero(),
            target,
            config,
            at_rest: false,
        }
    }

    /// Advance by one tick and return the current value.
    pub fn tick(&mut self) -> F {
        if self.at_rest { return self.value; }

        let displacement = self.value - self.target;
        let force = -self.config.stiffness * displacement - self.config.damping * self.velocity;
        let acceleration = force / self.config.mass;

        // Semi-implicit Euler: velocity first, then position
        self.velocity = self.velocity + acceleration;
        self.value = self.value + self.velocity;

        // Rest detection
        if self.velocity.abs() < self.config.rest_threshold
            && displacement.abs() < self.config.rest_threshold
        {
            self.value = self.target;
            self.velocity = F::zero();
            self.at_rest = true;
        }

        self.value
    }

    /// Current value.
    pub fn value(&self) -> F { self.value }

    /// Current velocity.
    pub fn velocity(&self) -> F { self.velocity }

    /// Whether the spring has settled.
    pub fn is_at_rest(&self) -> bool { self.at_rest }

    /// Change the target mid-flight. The spring smoothly redirects.
    /// Wakes the spring if it was at rest.
    pub fn set_target(&mut self, new_target: F) {
        self.target = new_target;
        self.at_rest = false;
    }

    /// Immediately set value and velocity (hard reset).
    pub fn reset(&mut self, value: F, target: F) {
        self.value = value;
        self.velocity = F::zero();
        self.target = target;
        self.at_rest = false;
    }
}
```

**Tests for Phase 5:**
- `spring_reaches_target` — after enough ticks, value ≈ target
- `spring_wobbly_overshoots` — wobbly spring goes past target before settling
- `spring_stiff_fast` — stiff spring settles faster than gentle
- `spring_at_rest` — is_at_rest() becomes true when settled
- `spring_retarget` — changing target mid-flight, spring redirects smoothly
- `spring_retarget_wakes` — setting target on resting spring wakes it
- `spring_deterministic` — same config + same ticks = same result
- `spring_reset` — hard reset returns to initial state

---

## Phase 6: Composition

### Step 12: Sequence (tween.rs or composition.rs)

```rust
/// Plays tweens one after another in order.
///
/// When the current tween finishes, advances to the next.
/// The overall animation finishes when the last tween finishes.
///
/// # Example
/// ```
/// use easel::{Sequence, Tween, Easing};
///
/// let mut seq = Sequence::new()
///     .push(Tween::new(0.0f32, 100.0, 30).with_easing(Easing::EaseOutCubic))
///     .push(Tween::new(100.0, 50.0, 20).with_easing(Easing::EaseInOutQuad));
/// ```
#[derive(Clone, Debug)]
pub struct Sequence<T: Lerp<F>, F: Float> {
    tweens: Vec<Tween<T, F>>,
    current_index: usize,
    state: TweenState,
    loop_mode: LoopMode,
    loops_completed: u32,
}

impl<T: Lerp<F> + Clone, F: Float> Sequence<T, F> {
    pub fn new() -> Self { /* ... */ }
    pub fn push(mut self, tween: Tween<T, F>) -> Self { /* ... */ }
    pub fn with_loop(mut self, mode: LoopMode) -> Self { /* ... */ }
    pub fn tick(&mut self) -> T { /* ... */ }
    pub fn value(&self) -> T { /* ... */ }
    pub fn total_duration(&self) -> u32 { /* ... */ }
    pub fn progress(&self) -> F { /* ... */ }
    pub fn is_finished(&self) -> bool { /* ... */ }
    pub fn reset(&mut self) { /* ... */ }
}
```

### Step 13: Parallel (tween.rs or composition.rs)

```rust
/// Plays multiple tweens simultaneously.
///
/// Finishes when the LONGEST tween finishes. Each tick returns the
/// current values of ALL tweens.
#[derive(Clone, Debug)]
pub struct Parallel<T: Lerp<F>, F: Float> {
    tweens: Vec<Tween<T, F>>,
    state: TweenState,
}

impl<T: Lerp<F> + Clone, F: Float> Parallel<T, F> {
    pub fn new() -> Self { /* ... */ }
    pub fn push(mut self, tween: Tween<T, F>) -> Self { /* ... */ }
    pub fn tick(&mut self) -> Vec<T> { /* ... */ }
    pub fn values(&self) -> Vec<T> { /* ... */ }
    pub fn is_finished(&self) -> bool { /* ... */ }
    pub fn total_duration(&self) -> u32 { /* max of all durations */ }
}
```

### Step 14: Stagger (tween.rs or composition.rs)

```rust
/// Like Parallel but each tween starts `offset` ticks after the previous.
///
/// Creates a cascading effect. E.g., 5 UI elements sliding in one after
/// another with a 5-tick delay between each.
#[derive(Clone, Debug)]
pub struct Stagger<T: Lerp<F>, F: Float> {
    tweens: Vec<Tween<T, F>>,
    offset: u32,
    elapsed: u32,
    state: TweenState,
}

impl<T: Lerp<F> + Clone, F: Float> Stagger<T, F> {
    pub fn new(offset: u32) -> Self { /* ... */ }
    pub fn push(mut self, tween: Tween<T, F>) -> Self { /* ... */ }
    pub fn tick(&mut self) -> Vec<T> { /* ... */ }
    pub fn values(&self) -> Vec<T> { /* ... */ }
    pub fn is_finished(&self) -> bool { /* ... */ }
    pub fn total_duration(&self) -> u32 {
        // last tween start + its duration
        // start of tween i = i * offset
    }
}
```

### Step 15: Timeline (timeline.rs)

```rust
/// A timing entry in the timeline.
#[derive(Clone, Debug)]
pub struct TimelineEntry {
    pub id: TweenId,
    pub start_tick: u32,
    pub duration: u32,
}

/// Heterogeneous animation timeline.
///
/// Unlike Sequence/Parallel (which own tweens of a specific type),
/// Timeline is a timing coordinator. It tells you WHEN each entry
/// is active and its local progress — the caller owns the actual
/// tween objects and uses TweenId to look them up.
///
/// # Example
/// ```
/// use easel::{Timeline, TweenId};
///
/// let mut tl = Timeline::new();
/// let pos_id = tl.add(0, 60);    // Position tween: ticks 0-60
/// let color_id = tl.add(30, 30); // Color tween: ticks 30-60
/// let scale_id = tl.add(10, 50); // Scale tween: ticks 10-60
///
/// for _ in 0..60 {
///     let active = tl.tick();
///     for (id, progress) in &active {
///         // Use id to look up and advance your tweens
///         match *id {
///             id if id == pos_id => { /* update position with progress */ }
///             id if id == color_id => { /* update color */ }
///             _ => {}
///         }
///     }
/// }
/// ```
#[derive(Clone, Debug)]
pub struct Timeline {
    entries: Vec<TimelineEntry>,
    next_id: u32,
    elapsed: u32,
    state: TweenState,
    loop_mode: LoopMode,
}

impl Timeline {
    pub fn new() -> Self { /* ... */ }

    /// Add an entry. Returns the TweenId for later lookup.
    pub fn add(&mut self, start_tick: u32, duration: u32) -> TweenId {
        let id = TweenId(self.next_id);
        self.next_id += 1;
        self.entries.push(TimelineEntry { id, start_tick, duration });
        id
    }

    /// Advance by one tick. Returns list of (TweenId, progress) for active entries.
    /// Progress is in [0.0, 1.0] representing local progress within that entry.
    pub fn tick<F: Float>(&mut self) -> Vec<(TweenId, F)> { /* ... */ }

    /// Total duration (end of last entry).
    pub fn total_duration(&self) -> u32 {
        self.entries.iter().map(|e| e.start_tick + e.duration).max().unwrap_or(0)
    }

    /// Seek to a specific tick.
    pub fn seek(&mut self, tick: u32) { self.elapsed = tick; }

    pub fn is_finished(&self) -> bool { /* ... */ }
    pub fn with_loop(mut self, mode: LoopMode) -> Self { /* ... */ }
    pub fn reset(&mut self) { /* ... */ }
}
```

**Tests for Phase 6:**
- `sequence_total_duration` — sum of all tween durations
- `sequence_plays_in_order` — first tween finishes before second starts
- `sequence_value_at_transition` — value matches second tween's start
- `parallel_finishes_with_longest` — finished when longest tween done
- `parallel_returns_all_values` — tick() returns correct count
- `stagger_offset` — second tween starts `offset` ticks after first
- `stagger_total_duration` — last start + last duration
- `timeline_active_entries` — only returns entries in their active range
- `timeline_progress` — local progress is correct for each entry
- `timeline_seek` — seek jumps to correct tick

---

## Phase 7: Config + Observer + Error

### Step 16: Config (config.rs)

```rust
/// CSS-like easing presets.
pub struct TweenConfig;

impl TweenConfig {
    /// CSS `ease`: cubic-bezier(0.25, 0.1, 0.25, 1.0)
    pub fn ease<F: Float>() -> Easing<F> {
        Easing::CubicBezier {
            x1: F::from_f32(0.25), y1: F::from_f32(0.1),
            x2: F::from_f32(0.25), y2: F::one(),
        }
    }

    /// CSS `ease-in`: cubic-bezier(0.42, 0, 1, 1)
    pub fn ease_in<F: Float>() -> Easing<F> {
        Easing::CubicBezier {
            x1: F::from_f32(0.42), y1: F::zero(),
            x2: F::one(), y2: F::one(),
        }
    }

    /// CSS `ease-out`: cubic-bezier(0, 0, 0.58, 1)
    pub fn ease_out<F: Float>() -> Easing<F> {
        Easing::CubicBezier {
            x1: F::zero(), y1: F::zero(),
            x2: F::from_f32(0.58), y2: F::one(),
        }
    }

    /// CSS `ease-in-out`: cubic-bezier(0.42, 0, 0.58, 1)
    pub fn ease_in_out<F: Float>() -> Easing<F> {
        Easing::CubicBezier {
            x1: F::from_f32(0.42), y1: F::zero(),
            x2: F::from_f32(0.58), y2: F::one(),
        }
    }

    /// CSS `linear`.
    pub fn linear<F: Float>() -> Easing<F> { Easing::Linear }

    /// Step function: instant snap to target.
    pub fn snap<F: Float>() -> Easing<F> {
        Easing::CubicBezier {
            x1: F::zero(), y1: F::one(),
            x2: F::zero(), y2: F::one(),
        }
    }
}
```

### Step 17: Observer (observer.rs)

```rust
/// Trait for observing tween lifecycle events.
pub trait TweenObserver {
    fn on_start(&mut self, _id: TweenId) {}
    fn on_complete(&mut self, _id: TweenId) {}
    fn on_loop(&mut self, _id: TweenId, _count: u32) {}
    fn on_pause(&mut self, _id: TweenId) {}
    fn on_resume(&mut self, _id: TweenId) {}
}

pub struct NoOpObserver;
impl TweenObserver for NoOpObserver {}
```

### Step 18: Error (error.rs)

```rust
/// Errors that can occur during tween construction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TweenError {
    /// Keyframes list is empty.
    EmptyKeyframes,
    /// Duration is zero.
    InvalidDuration,
    /// Keyframes are not in ascending tick order.
    KeyframeOutOfOrder { index: usize, tick: u32, prev_tick: u32 },
    /// Cubic bezier control point x is outside [0, 1].
    InvalidBezierControl,
}
```

**Tests for Phase 7:**
- `config_ease_matches_css` — CSS ease bezier control points correct
- `config_snap` — snap easing jumps to 1.0 immediately
- `observer_noop_compiles` — NoOpObserver works as default
- `error_variants` — all error variants constructible

---

## Phase 8: Integration + lib.rs + Tests

### Step 19: lib.rs Re-exports

```rust
#![no_std]
extern crate alloc;

pub mod float;
pub mod lerp;
pub mod easing;
pub mod tween;
pub mod keyframes;
pub mod spring;
pub mod timeline;
pub mod loop_mode;
pub mod state;
pub mod observer;
pub mod error;
pub mod config;

// Re-exports
pub use float::Float;
pub use lerp::{Lerp, Rgba, Angle};
pub use easing::Easing;
pub use tween::{Tween, TweenId};
pub use keyframes::{Keyframe, Keyframes};
pub use spring::{SpringTween, SpringConfig};
pub use timeline::Timeline;
pub use loop_mode::LoopMode;
pub use state::TweenState;
pub use observer::{TweenObserver, NoOpObserver};
pub use error::TweenError;
pub use config::TweenConfig;
```

### Step 20: Full Test List

```text
# float.rs (4 tests)
- float_f32_basics, float_f32_math
- float_f64_basics, float_f64_math

# lerp.rs (10 tests)
- lerp_scalar_midpoint, lerp_scalar_endpoints
- lerp_tuple2, lerp_tuple3, lerp_tuple4
- lerp_array
- lerp_rgba_opaque, lerp_rgba_transparent
- lerp_angle_short_path, lerp_angle_half

# easing.rs (12 tests)
- all_easings_at_zero, all_easings_at_one
- linear_is_identity
- quad_in_slow_start, quad_out_fast_start
- cubic_bezier_linear, cubic_bezier_css_ease
- back_overshoots, elastic_overshoots
- bounce_never_negative
- in_out_symmetry
- in_out_quad_midpoint

# tween.rs (12 tests)
- tween_basic_linear, tween_easing_applied
- tween_delay, tween_finishes
- tween_loop_count, tween_loop_infinite
- tween_pingpong, tween_pingpong_midpoint
- tween_pause_resume, tween_reset
- tween_retarget, tween_progress

# keyframes.rs (4 tests)
- keyframes_two_point, keyframes_three_point
- keyframes_different_easings, keyframes_loop

# spring.rs (7 tests)
- spring_reaches_target, spring_wobbly_overshoots
- spring_stiff_fast, spring_at_rest
- spring_retarget, spring_retarget_wakes
- spring_deterministic

# composition (6 tests)
- sequence_total_duration, sequence_plays_in_order
- parallel_finishes_with_longest
- stagger_offset, stagger_total_duration
- timeline_active_entries

# timeline.rs (3 tests)
- timeline_progress, timeline_seek, timeline_loop

# config.rs (2 tests)
- config_ease_matches_css, config_snap

# integration (3 tests)
- integration_ui_slide_in
- integration_color_fade
- integration_camera_spring
```

---

## Phase 9: WASM Demo

The demo should have 4 tabs, each showcasing a different feature.

#### Tab 1: Easing Curves
- Display all 30 easing functions as small curve graphs in a grid
- Click any curve to select it
- Selected curve animates a ball from left to right
- Progress bar below shows normalized time
- Curve graph highlights the current point

#### Tab 2: Tween Playground
- Two input fields: "from" and "to" values
- Duration slider (10-120 ticks)
- Easing dropdown with all 30 options
- Loop mode toggle (Once, Infinite, PingPong)
- Delay slider (0-60 ticks)
- Play / Pause / Reset buttons
- Animated square moving from `from` to `to`
- Progress bar and value readout

#### Tab 3: Spring Physics
- A circle follows the mouse cursor via spring physics
- Preset dropdown: Gentle, Wobbly, Stiff, Slow, Molasses
- Custom sliders: stiffness, damping, mass
- Real-time graphs: value vs time, velocity vs time
- Reset button to snap back to center
- Shows "at rest" indicator when settled

#### Tab 4: Timeline Editor
- Visual timeline with horizontal tracks
- Draggable blocks representing tween entries
- Play head that advances with ticks
- Loop mode toggle
- 3 animated objects (square, circle, triangle) each on their own track
- Blocks show easing curve preview
- Click to add/remove entries

**WASM bindings pattern** (in `demo-wasm/src/lib.rs`):
- Export `Demo` struct wrapping active tweens and springs
- Export `tick()`, `render_state()` functions
- Return flat arrays of render data to JS

**JS pattern** (in `demo-wasm/www/main.js`):
- Canvas 2D rendering for animated objects and curve graphs
- Input controls for tween parameters
- `requestAnimationFrame` loop

---

## Algorithm References

### All 30 Penner Easing Functions

```text
=== Polynomial ===

Linear:          t
EaseInQuad:      t²
EaseOutQuad:     1 - (1-t)²
EaseInOutQuad:   t < 0.5 ? 2t² : 1 - (-2t+2)²/2

EaseInCubic:     t³
EaseOutCubic:    1 - (1-t)³
EaseInOutCubic:  t < 0.5 ? 4t³ : 1 - (-2t+2)³/2

EaseInQuart:     t⁴
EaseOutQuart:    1 - (1-t)⁴
EaseInOutQuart:  t < 0.5 ? 8t⁴ : 1 - (-2t+2)⁴/2

EaseInQuint:     t⁵
EaseOutQuint:    1 - (1-t)⁵
EaseInOutQuint:  t < 0.5 ? 16t⁵ : 1 - (-2t+2)⁵/2

=== Trigonometric ===

EaseInSine:      1 - cos(t × π/2)
EaseOutSine:     sin(t × π/2)
EaseInOutSine:   -(cos(πt) - 1) / 2

=== Exponential ===

EaseInExpo:      t == 0 ? 0 : 2^(10t - 10)
EaseOutExpo:     t == 1 ? 1 : 1 - 2^(-10t)
EaseInOutExpo:   t == 0 ? 0 : t == 1 ? 1 :
                 t < 0.5 ? 2^(20t - 10)/2 : (2 - 2^(-20t + 10))/2

=== Circular ===

EaseInCirc:      1 - sqrt(1 - t²)
EaseOutCirc:     sqrt(1 - (t-1)²)
EaseInOutCirc:   t < 0.5 ? (1 - sqrt(1 - (2t)²)) / 2
                          : (sqrt(1 - (-2t+2)²) + 1) / 2

=== Back (overshoot) ===
c1 = 1.70158
c2 = c1 * 1.525
c3 = c1 + 1

EaseInBack:      c3×t³ - c1×t²
EaseOutBack:     1 + c3×(t-1)³ + c1×(t-1)²
EaseInOutBack:   t < 0.5 ? ((2t)² × ((c2+1)×2t - c2)) / 2
                          : ((2t-2)² × ((c2+1)×(2t-2) + c2) + 2) / 2

=== Elastic ===
c4 = 2π/3
c5 = 2π/4.5

EaseInElastic:   t == 0|1 ? t :
                 -2^(10t-10) × sin((10t - 10.75) × c4)
EaseOutElastic:  t == 0|1 ? t :
                 2^(-10t) × sin((10t - 0.75) × c4) + 1
EaseInOutElastic: t == 0|1 ? t :
                  t < 0.5 ? -(2^(20t-10) × sin((20t - 11.125) × c5)) / 2
                           : (2^(-20t+10) × sin((20t - 11.125) × c5)) / 2 + 1

=== Bounce ===
n1 = 7.5625
d1 = 2.75

EaseOutBounce:
    t < 1/d1       → n1 × t²
    t < 2/d1       → n1 × (t - 1.5/d1)² + 0.75
    t < 2.5/d1     → n1 × (t - 2.25/d1)² + 0.9375
    else            → n1 × (t - 2.625/d1)² + 0.984375

EaseInBounce:      1 - EaseOutBounce(1-t)
EaseInOutBounce:   t < 0.5 ? (1 - EaseOutBounce(1 - 2t)) / 2
                            : (1 + EaseOutBounce(2t - 1)) / 2
```

### Cubic Bezier Evaluation (Newton-Raphson)

```text
Given control points P0=(0,0), P1=(x1,y1), P2=(x2,y2), P3=(1,1):

Bezier parametric form (one component):
    B(s) = 3(1-s)²s × P1 + 3(1-s)s² × P2 + s³
    (Note: P0=0 and P3=1 are baked into this simplified form)

B'(s) = 3(1-s)² × P1 + 6(1-s)s × (P2-P1) + 3s² × (1-P2)

To find y for input x = t:
    1. Solve B_x(s) = t for s:
        s₀ = t (initial guess)
        Repeat 8 times:
            s = s - (B_x(s) - t) / B_x'(s)
            Clamp s to [0, 1]
        If |B_x'(s)| < epsilon: bisection fallback

    2. Return B_y(s)
```

### Semi-Implicit Euler Spring

```text
Physics model: spring-damper system
    F_spring = -k × (x - x_target)     // Hooke's law
    F_damping = -c × v                   // Viscous damping
    F_total = F_spring + F_damping

    a = F_total / mass

Semi-implicit Euler (symplectic):
    v_{n+1} = v_n + a × dt              // Update velocity FIRST
    x_{n+1} = x_n + v_{n+1} × dt       // Then update position with NEW velocity
    (dt = 1 for tick-based, baked into stiffness/damping values)

Why not standard Euler?
    Standard: x_new = x + v*dt, v_new = v + a*dt
    This adds energy over time → unstable oscillation
    Semi-implicit: slightly dissipative → naturally stable

Rest detection:
    |v| < threshold AND |x - target| < threshold
    → Snap to target, zero velocity, mark at_rest
```

### Angular Interpolation (Shortest Path)

```text
Problem: Lerping from 350° to 10° naively goes through 180° (the long way)

Solution:
    diff = to - from                    // 10 - 350 = -340
    Normalize to [-180, 180]:
        while diff > 180: diff -= 360   // -340 + 360 = 20
        while diff < -180: diff += 360
    result = from + diff × t            // 350 + 20 × 0.5 = 360 = 0°

In radians:
    Normalize to [-π, π] using the same approach
```

### Premultiplied Alpha Color Lerp

```text
Problem: Standard lerp of RGBA(1,0,0,1) → RGBA(0,0,0,0) passes through
         dark semi-transparent red, creating visible dark fringes.

Solution: Premultiplied alpha interpolation
    1. Premultiply: (r×a, g×a, b×a, a)
    2. Lerp in premultiplied space (component-wise)
    3. Unpremultiply: (r/a, g/a, b/a, a)

This interpolates the visual contribution directly, avoiding
the fringe artifact.
```

---

## Verification Checklist

```bash
# In easel/
cargo test --target x86_64-pc-windows-msvc
cargo build --target wasm32-unknown-unknown --release
cargo clippy --target x86_64-pc-windows-msvc

# WASM demo
cd demo-wasm
wasm-pack build --target web --release
# Serve demo-wasm/www/ with a local HTTP server and test in browser
```
