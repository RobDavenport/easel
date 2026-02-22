/// Errors that can occur during tween construction/validation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TweenError {
    /// Keyframes list is empty.
    EmptyKeyframes,
    /// Duration is zero.
    InvalidDuration,
    /// Keyframes are not in ascending tick order.
    KeyframeOutOfOrder {
        index: usize,
        tick: u32,
        prev_tick: u32,
    },
    /// Cubic bezier control point x is outside [0, 1].
    InvalidBezierControl,
}

#[cfg(test)]
mod tests {
    use crate::error::TweenError;

    #[test]
    fn error_variants() {
        let _ = TweenError::EmptyKeyframes;
        let _ = TweenError::InvalidDuration;
        let _ = TweenError::KeyframeOutOfOrder {
            index: 1,
            tick: 5,
            prev_tick: 10,
        };
        let _ = TweenError::InvalidBezierControl;
    }
}
