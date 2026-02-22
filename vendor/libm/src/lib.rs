#![no_std]

extern "C" {
    #[link_name = "sqrtf"]
    fn c_sqrtf(x: f32) -> f32;
    #[link_name = "sinf"]
    fn c_sinf(x: f32) -> f32;
    #[link_name = "cosf"]
    fn c_cosf(x: f32) -> f32;
    #[link_name = "powf"]
    fn c_powf(x: f32, y: f32) -> f32;
    #[link_name = "expf"]
    fn c_expf(x: f32) -> f32;
    #[link_name = "floorf"]
    fn c_floorf(x: f32) -> f32;

    #[link_name = "sqrt"]
    fn c_sqrt(x: f64) -> f64;
    #[link_name = "sin"]
    fn c_sin(x: f64) -> f64;
    #[link_name = "cos"]
    fn c_cos(x: f64) -> f64;
    #[link_name = "pow"]
    fn c_pow(x: f64, y: f64) -> f64;
    #[link_name = "exp"]
    fn c_exp(x: f64) -> f64;
    #[link_name = "floor"]
    fn c_floor(x: f64) -> f64;
}

#[inline]
pub fn sqrtf(x: f32) -> f32 {
    // SAFETY: direct FFI call to C runtime math routine.
    unsafe { c_sqrtf(x) }
}

#[inline]
pub fn sinf(x: f32) -> f32 {
    // SAFETY: direct FFI call to C runtime math routine.
    unsafe { c_sinf(x) }
}

#[inline]
pub fn cosf(x: f32) -> f32 {
    // SAFETY: direct FFI call to C runtime math routine.
    unsafe { c_cosf(x) }
}

#[inline]
pub fn fabsf(x: f32) -> f32 {
    if x < 0.0 { -x } else { x }
}

#[inline]
pub fn powf(x: f32, y: f32) -> f32 {
    // SAFETY: direct FFI call to C runtime math routine.
    unsafe { c_powf(x, y) }
}

#[inline]
pub fn expf(x: f32) -> f32 {
    // SAFETY: direct FFI call to C runtime math routine.
    unsafe { c_expf(x) }
}

#[inline]
pub fn floorf(x: f32) -> f32 {
    // SAFETY: direct FFI call to C runtime math routine.
    unsafe { c_floorf(x) }
}

#[inline]
pub fn sqrt(x: f64) -> f64 {
    // SAFETY: direct FFI call to C runtime math routine.
    unsafe { c_sqrt(x) }
}

#[inline]
pub fn sin(x: f64) -> f64 {
    // SAFETY: direct FFI call to C runtime math routine.
    unsafe { c_sin(x) }
}

#[inline]
pub fn cos(x: f64) -> f64 {
    // SAFETY: direct FFI call to C runtime math routine.
    unsafe { c_cos(x) }
}

#[inline]
pub fn fabs(x: f64) -> f64 {
    if x < 0.0 { -x } else { x }
}

#[inline]
pub fn pow(x: f64, y: f64) -> f64 {
    // SAFETY: direct FFI call to C runtime math routine.
    unsafe { c_pow(x, y) }
}

#[inline]
pub fn exp(x: f64) -> f64 {
    // SAFETY: direct FFI call to C runtime math routine.
    unsafe { c_exp(x) }
}

#[inline]
pub fn floor(x: f64) -> f64 {
    // SAFETY: direct FFI call to C runtime math routine.
    unsafe { c_floor(x) }
}
