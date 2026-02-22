use alloc::vec::Vec;

use crate::easing::Easing;
use crate::error::TweenError;
use crate::float::Float;
use crate::lerp::Lerp;
use crate::loop_mode::LoopMode;
use crate::state::TweenState;

/// A single point in a keyframed animation.
#[derive(Clone, Debug)]
pub struct Keyframe<T: Lerp<F>, F: Float> {
    /// The value at this keyframe.
    pub value: T,
    /// Tick position from start.
    pub tick: u32,
    /// Easing from this keyframe to the next.
    pub easing: Easing<F>,
}

/// Multi-point animation with per-segment easing.
#[derive(Clone, Debug)]
pub struct Keyframes<T: Lerp<F>, F: Float> {
    frames: Vec<Keyframe<T, F>>,
    elapsed: u32,
    state: TweenState,
    loop_mode: LoopMode,
    loops_completed: u32,
}

impl<T: Lerp<F> + Clone, F: Float> Keyframes<T, F> {
    pub fn new(frames: Vec<Keyframe<T, F>>) -> Self {
        Self::try_new(frames).expect("invalid keyframes")
    }

    pub fn try_new(frames: Vec<Keyframe<T, F>>) -> Result<Self, TweenError> {
        validate_frames(&frames)?;
        Ok(Self {
            frames,
            elapsed: 0,
            state: TweenState::Playing,
            loop_mode: LoopMode::Once,
            loops_completed: 0,
        })
    }

    pub fn with_loop(mut self, mode: LoopMode) -> Self {
        self.loop_mode = mode;
        self
    }

    /// Advance by one tick and return interpolated value.
    pub fn tick(&mut self) -> T {
        assert!(!self.frames.is_empty(), "Keyframes cannot be empty");
        if self.state != TweenState::Playing {
            return self.value();
        }

        let total = self.total_duration();
        if total > 0 && self.elapsed < total {
            self.elapsed += 1;
        }

        let value = self.value();
        if self.elapsed >= total {
            self.on_iteration_complete();
        }
        value
    }

    pub fn value(&self) -> T {
        assert!(!self.frames.is_empty(), "Keyframes cannot be empty");
        if self.frames.len() == 1 {
            return self.frames[0].value.clone();
        }

        if self.elapsed <= self.frames[0].tick {
            return self.frames[0].value.clone();
        }

        let last = self.frames.len() - 1;
        if self.elapsed >= self.frames[last].tick {
            return self.frames[last].value.clone();
        }

        let idx = self
            .frames
            .partition_point(|frame| frame.tick <= self.elapsed);
        let i = idx.saturating_sub(1);
        let a = &self.frames[i];
        let b = &self.frames[i + 1];
        let segment_duration = b.tick.saturating_sub(a.tick);
        if segment_duration == 0 {
            return b.value.clone();
        }

        let local_elapsed = self.elapsed.saturating_sub(a.tick);
        let raw_t = F::from_f32(local_elapsed as f32 / segment_duration as f32);
        let eased_t = a.easing.evaluate(raw_t);
        a.value.lerp(&b.value, eased_t)
    }

    pub fn total_duration(&self) -> u32 {
        self.frames.last().map(|f| f.tick).unwrap_or(0)
    }

    pub fn progress(&self) -> F {
        let total = self.total_duration();
        if total == 0 {
            return F::one();
        }
        F::from_f32(self.elapsed.min(total) as f32 / total as f32)
    }

    pub fn is_finished(&self) -> bool {
        self.state == TweenState::Finished
    }

    pub fn reset(&mut self) {
        self.elapsed = 0;
        self.state = TweenState::Playing;
        self.loops_completed = 0;
    }

    fn on_iteration_complete(&mut self) {
        match self.loop_mode {
            LoopMode::Once => {
                self.state = TweenState::Finished;
            }
            LoopMode::Count(count) => {
                self.loops_completed += 1;
                if count == 0 || self.loops_completed >= count {
                    self.state = TweenState::Finished;
                } else {
                    self.elapsed = 0;
                }
            }
            LoopMode::Infinite | LoopMode::PingPong => {
                self.loops_completed += 1;
                self.elapsed = 0;
            }
            LoopMode::PingPongCount(count) => {
                self.loops_completed += 1;
                let max_legs = count.saturating_mul(2);
                if max_legs == 0 || self.loops_completed >= max_legs {
                    self.state = TweenState::Finished;
                } else {
                    self.elapsed = 0;
                }
            }
        }
    }
}

fn validate_frames<T: Lerp<F>, F: Float>(frames: &[Keyframe<T, F>]) -> Result<(), TweenError> {
    if frames.is_empty() {
        return Err(TweenError::EmptyKeyframes);
    }
    for (i, window) in frames.windows(2).enumerate() {
        let prev = window[0].tick;
        let next = window[1].tick;
        if next < prev {
            return Err(TweenError::KeyframeOutOfOrder {
                index: i + 1,
                tick: next,
                prev_tick: prev,
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use alloc::vec;

    use crate::easing::Easing;
    use crate::keyframes::{Keyframe, Keyframes};
    use crate::loop_mode::LoopMode;

    const EPS: f32 = 1e-4;

    fn approx(a: f32, b: f32) -> bool {
        (a - b).abs() < EPS
    }

    #[test]
    fn keyframes_two_point() {
        let mut keyframes = Keyframes::new(vec![
            Keyframe {
                value: 0.0f32,
                tick: 0,
                easing: Easing::Linear,
            },
            Keyframe {
                value: 100.0f32,
                tick: 10,
                easing: Easing::Linear,
            },
        ]);
        let mut value = 0.0;
        for _ in 0..5 {
            value = keyframes.tick();
        }
        assert!(approx(value, 50.0));
    }

    #[test]
    fn keyframes_three_point() {
        let mut keyframes = Keyframes::new(vec![
            Keyframe {
                value: 0.0f32,
                tick: 0,
                easing: Easing::Linear,
            },
            Keyframe {
                value: 100.0f32,
                tick: 5,
                easing: Easing::Linear,
            },
            Keyframe {
                value: 50.0f32,
                tick: 10,
                easing: Easing::Linear,
            },
        ]);
        let mut value = 0.0;
        for _ in 0..5 {
            value = keyframes.tick();
        }
        assert!(approx(value, 100.0));
    }

    #[test]
    fn keyframes_different_easings() {
        let mut keyframes = Keyframes::new(vec![
            Keyframe {
                value: 0.0f32,
                tick: 0,
                easing: Easing::EaseInQuad,
            },
            Keyframe {
                value: 100.0f32,
                tick: 10,
                easing: Easing::EaseOutQuad,
            },
            Keyframe {
                value: 0.0f32,
                tick: 20,
                easing: Easing::Linear,
            },
        ]);

        for _ in 0..5 {
            keyframes.tick();
        }
        let first_mid = keyframes.value();
        for _ in 0..10 {
            keyframes.tick();
        }
        let second_mid = keyframes.value();
        assert!(first_mid < 50.0);
        assert!(second_mid < 50.0);
    }

    #[test]
    fn keyframes_loop() {
        let mut keyframes = Keyframes::new(vec![
            Keyframe {
                value: 0.0f32,
                tick: 0,
                easing: Easing::Linear,
            },
            Keyframe {
                value: 10.0f32,
                tick: 2,
                easing: Easing::Linear,
            },
        ])
        .with_loop(LoopMode::Infinite);

        for _ in 0..10 {
            keyframes.tick();
        }
        assert!(!keyframes.is_finished());
    }
}
