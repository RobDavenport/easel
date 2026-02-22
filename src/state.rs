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
