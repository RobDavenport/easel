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
    /// Play forward, then backward, and repeat.
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
