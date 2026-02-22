use alloc::vec::Vec;

use crate::easing::Easing;
use crate::float::Float;
use crate::lerp::Lerp;
use crate::loop_mode::{LoopMode, PlayDirection};
use crate::state::TweenState;

/// Opaque identifier for a tween in a Timeline.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TweenId(pub u32);

/// A single from-to animation with easing, delay, and looping.
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
    pub fn new(from: T, to: T, duration: u32) -> Self {
        Self {
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

    /// Set the easing function.
    pub fn with_easing(mut self, easing: Easing<F>) -> Self {
        self.easing = easing;
        self
    }

    /// Set loop mode.
    pub fn with_loop(mut self, mode: LoopMode) -> Self {
        self.loop_mode = mode;
        self
    }

    /// Set delay in ticks before animation starts.
    pub fn with_delay(mut self, ticks: u32) -> Self {
        self.delay = ticks;
        self.delay_remaining = ticks;
        self
    }

    /// Advance by one tick and return current value.
    pub fn tick(&mut self) -> T {
        if self.state != TweenState::Playing {
            return self.value();
        }

        if self.delay_remaining > 0 {
            self.delay_remaining -= 1;
            return self.from.clone();
        }

        if self.duration == 0 {
            self.state = TweenState::Finished;
            return match self.direction {
                PlayDirection::Forward => self.to.clone(),
                PlayDirection::Backward => self.from.clone(),
            };
        }

        if self.elapsed < self.duration {
            self.elapsed += 1;
        }

        let value = self.value();

        if self.elapsed >= self.duration {
            self.on_iteration_complete();
        }

        value
    }

    /// Get current value without advancing.
    pub fn value(&self) -> T {
        if self.delay_remaining > 0 {
            return self.from.clone();
        }

        if self.duration == 0 {
            return match self.direction {
                PlayDirection::Forward => self.to.clone(),
                PlayDirection::Backward => self.from.clone(),
            };
        }

        let eased = self.easing.evaluate(self.progress());
        self.from.lerp(&self.to, eased)
    }

    /// Normalized progress [0, 1] within current iteration.
    pub fn progress(&self) -> F {
        if self.delay_remaining > 0 {
            return F::zero();
        }

        if self.duration == 0 {
            return F::one();
        }

        let raw = (self.elapsed as f32 / self.duration as f32).clamp(0.0, 1.0);
        let progress = F::from_f32(raw);
        match self.direction {
            PlayDirection::Forward => progress,
            PlayDirection::Backward => F::one() - progress,
        }
    }

    /// Whether completed all iterations.
    pub fn is_finished(&self) -> bool {
        self.state == TweenState::Finished
    }

    /// Current state.
    pub fn state(&self) -> TweenState {
        self.state
    }

    /// Reset to initial state.
    pub fn reset(&mut self) {
        self.elapsed = 0;
        self.delay_remaining = self.delay;
        self.loops_completed = 0;
        self.direction = PlayDirection::Forward;
        self.state = TweenState::Playing;
    }

    /// Pause animation.
    pub fn pause(&mut self) {
        if self.state == TweenState::Playing {
            self.state = TweenState::Paused;
        }
    }

    /// Resume paused animation.
    pub fn resume(&mut self) {
        if self.state == TweenState::Paused {
            self.state = TweenState::Playing;
        }
    }

    /// Change target value mid-flight.
    pub fn set_target(&mut self, new_to: T) {
        self.to = new_to;
    }

    /// Change both ends mid-flight.
    pub fn set_range(&mut self, new_from: T, new_to: T) {
        self.from = new_from;
        self.to = new_to;
    }

    /// Total duration including delay.
    pub fn total_duration(&self) -> u32 {
        self.delay + self.duration
    }

    /// Loops completed so far.
    pub fn loops_completed(&self) -> u32 {
        self.loops_completed
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
                    self.direction = PlayDirection::Forward;
                }
            }
            LoopMode::Infinite => {
                self.loops_completed += 1;
                self.elapsed = 0;
            }
            LoopMode::PingPong => {
                self.loops_completed += 1;
                self.elapsed = 0;
                self.direction = match self.direction {
                    PlayDirection::Forward => PlayDirection::Backward,
                    PlayDirection::Backward => PlayDirection::Forward,
                };
            }
            LoopMode::PingPongCount(count) => {
                self.loops_completed += 1;
                let max_legs = count.saturating_mul(2);
                if max_legs == 0 || self.loops_completed >= max_legs {
                    self.state = TweenState::Finished;
                } else {
                    self.elapsed = 0;
                    self.direction = match self.direction {
                        PlayDirection::Forward => PlayDirection::Backward,
                        PlayDirection::Backward => PlayDirection::Forward,
                    };
                }
            }
        }
    }
}

/// Plays tweens one after another in order.
#[derive(Clone, Debug)]
pub struct Sequence<T: Lerp<F>, F: Float> {
    tweens: Vec<Tween<T, F>>,
    current_index: usize,
    state: TweenState,
    loop_mode: LoopMode,
    loops_completed: u32,
}

impl<T: Lerp<F> + Clone, F: Float> Sequence<T, F> {
    pub fn new() -> Self {
        Self {
            tweens: Vec::new(),
            current_index: 0,
            state: TweenState::Idle,
            loop_mode: LoopMode::Once,
            loops_completed: 0,
        }
    }

    pub fn push(mut self, tween: Tween<T, F>) -> Self {
        self.tweens.push(tween);
        if self.state == TweenState::Idle {
            self.state = TweenState::Playing;
        }
        self
    }

    pub fn with_loop(mut self, mode: LoopMode) -> Self {
        self.loop_mode = mode;
        self
    }

    pub fn tick(&mut self) -> T {
        assert!(
            !self.tweens.is_empty(),
            "Sequence requires at least one tween"
        );
        if self.state != TweenState::Playing {
            return self.value();
        }

        let value = self.tweens[self.current_index].tick();
        if self.tweens[self.current_index].is_finished() {
            if self.current_index + 1 < self.tweens.len() {
                self.current_index += 1;
            } else {
                self.on_sequence_complete();
            }
        }
        value
    }

    pub fn value(&self) -> T {
        assert!(
            !self.tweens.is_empty(),
            "Sequence requires at least one tween"
        );
        self.tweens[self.current_index].value()
    }

    pub fn total_duration(&self) -> u32 {
        self.tweens.iter().map(Tween::total_duration).sum()
    }

    pub fn progress(&self) -> F {
        let total = self.total_duration();
        if total == 0 {
            return F::one();
        }
        let mut elapsed_ticks = 0u32;
        for (idx, tween) in self.tweens.iter().enumerate() {
            if idx < self.current_index {
                elapsed_ticks = elapsed_ticks.saturating_add(tween.total_duration());
            } else if idx == self.current_index {
                let local = tween.progress().to_f32() * tween.total_duration() as f32;
                elapsed_ticks = elapsed_ticks.saturating_add(local as u32);
            }
        }
        F::from_f32(elapsed_ticks as f32 / total as f32).clamp(F::zero(), F::one())
    }

    pub fn is_finished(&self) -> bool {
        self.state == TweenState::Finished
    }

    pub fn reset(&mut self) {
        for tween in &mut self.tweens {
            tween.reset();
        }
        self.current_index = 0;
        self.state = if self.tweens.is_empty() {
            TweenState::Idle
        } else {
            TweenState::Playing
        };
        self.loops_completed = 0;
    }

    fn on_sequence_complete(&mut self) {
        match self.loop_mode {
            LoopMode::Once => {
                self.state = TweenState::Finished;
            }
            LoopMode::Count(count) => {
                self.loops_completed += 1;
                if count == 0 || self.loops_completed >= count {
                    self.state = TweenState::Finished;
                } else {
                    self.restart();
                }
            }
            LoopMode::Infinite | LoopMode::PingPong => {
                self.loops_completed += 1;
                self.restart();
            }
            LoopMode::PingPongCount(count) => {
                self.loops_completed += 1;
                let max_legs = count.saturating_mul(2);
                if max_legs == 0 || self.loops_completed >= max_legs {
                    self.state = TweenState::Finished;
                } else {
                    self.restart();
                }
            }
        }
    }

    fn restart(&mut self) {
        for tween in &mut self.tweens {
            tween.reset();
        }
        self.current_index = 0;
        self.state = TweenState::Playing;
    }
}

impl<T: Lerp<F> + Clone, F: Float> Default for Sequence<T, F> {
    fn default() -> Self {
        Self::new()
    }
}

/// Plays multiple tweens simultaneously.
#[derive(Clone, Debug)]
pub struct Parallel<T: Lerp<F>, F: Float> {
    tweens: Vec<Tween<T, F>>,
    state: TweenState,
}

impl<T: Lerp<F> + Clone, F: Float> Parallel<T, F> {
    pub fn new() -> Self {
        Self {
            tweens: Vec::new(),
            state: TweenState::Idle,
        }
    }

    pub fn push(mut self, tween: Tween<T, F>) -> Self {
        self.tweens.push(tween);
        if self.state == TweenState::Idle {
            self.state = TweenState::Playing;
        }
        self
    }

    pub fn tick(&mut self) -> Vec<T> {
        if self.state != TweenState::Playing {
            return self.values();
        }

        let values: Vec<T> = self.tweens.iter_mut().map(Tween::tick).collect();
        if self.tweens.iter().all(Tween::is_finished) {
            self.state = TweenState::Finished;
        }
        values
    }

    pub fn values(&self) -> Vec<T> {
        self.tweens.iter().map(Tween::value).collect()
    }

    pub fn is_finished(&self) -> bool {
        self.state == TweenState::Finished
    }

    pub fn total_duration(&self) -> u32 {
        self.tweens
            .iter()
            .map(Tween::total_duration)
            .max()
            .unwrap_or(0)
    }
}

impl<T: Lerp<F> + Clone, F: Float> Default for Parallel<T, F> {
    fn default() -> Self {
        Self::new()
    }
}

/// Like parallel but each tween starts with a fixed tick offset.
#[derive(Clone, Debug)]
pub struct Stagger<T: Lerp<F>, F: Float> {
    tweens: Vec<Tween<T, F>>,
    offset: u32,
    elapsed: u32,
    state: TweenState,
}

impl<T: Lerp<F> + Clone, F: Float> Stagger<T, F> {
    pub fn new(offset: u32) -> Self {
        Self {
            tweens: Vec::new(),
            offset,
            elapsed: 0,
            state: TweenState::Idle,
        }
    }

    pub fn push(mut self, tween: Tween<T, F>) -> Self {
        self.tweens.push(tween);
        if self.state == TweenState::Idle {
            self.state = TweenState::Playing;
        }
        self
    }

    pub fn tick(&mut self) -> Vec<T> {
        if self.state != TweenState::Playing {
            return self.values();
        }

        let mut values = Vec::with_capacity(self.tweens.len());
        for (idx, tween) in self.tweens.iter_mut().enumerate() {
            let start = (idx as u32).saturating_mul(self.offset);
            if self.elapsed >= start {
                values.push(tween.tick());
            } else {
                values.push(tween.value());
            }
        }

        if self.tweens.iter().all(Tween::is_finished) {
            self.state = TweenState::Finished;
        } else {
            self.elapsed = self.elapsed.saturating_add(1);
        }

        values
    }

    pub fn values(&self) -> Vec<T> {
        self.tweens.iter().map(Tween::value).collect()
    }

    pub fn is_finished(&self) -> bool {
        self.state == TweenState::Finished
    }

    pub fn total_duration(&self) -> u32 {
        self.tweens
            .iter()
            .enumerate()
            .map(|(idx, tween)| (idx as u32).saturating_mul(self.offset) + tween.total_duration())
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::{Parallel, Sequence, Stagger, Tween};
    use crate::{Easing, LoopMode, TweenState};

    const EPS: f32 = 1e-4;

    fn approx(a: f32, b: f32) -> bool {
        (a - b).abs() < EPS
    }

    #[test]
    fn tween_basic_linear() {
        let mut tween = Tween::new(0.0f32, 100.0, 10);
        let mut value = 0.0;
        for _ in 0..5 {
            value = tween.tick();
        }
        assert!(approx(value, 50.0));
    }

    #[test]
    fn tween_easing_applied() {
        let mut tween = Tween::new(0.0f32, 100.0, 10).with_easing(Easing::EaseInQuad);
        let mut value = 0.0;
        for _ in 0..5 {
            value = tween.tick();
        }
        assert!(value < 50.0);
    }

    #[test]
    fn tween_delay() {
        let mut tween = Tween::new(0.0f32, 100.0, 10).with_delay(2);
        assert!(approx(tween.tick(), 0.0));
        assert!(approx(tween.tick(), 0.0));
        assert!(tween.tick() > 0.0);
    }

    #[test]
    fn tween_finishes() {
        let mut tween = Tween::new(0.0f32, 1.0, 10);
        for _ in 0..10 {
            tween.tick();
        }
        assert_eq!(tween.state(), TweenState::Finished);
    }

    #[test]
    fn tween_loop_count() {
        let mut tween = Tween::new(0.0f32, 1.0, 2).with_loop(LoopMode::Count(3));
        for _ in 0..6 {
            tween.tick();
        }
        assert!(tween.is_finished());
        assert_eq!(tween.loops_completed(), 3);
    }

    #[test]
    fn tween_loop_infinite() {
        let mut tween = Tween::new(0.0f32, 1.0, 2).with_loop(LoopMode::Infinite);
        for _ in 0..1000 {
            tween.tick();
        }
        assert!(!tween.is_finished());
    }

    #[test]
    fn tween_pingpong() {
        let mut tween = Tween::new(0.0f32, 10.0, 4).with_loop(LoopMode::PingPong);
        let mut values = [0.0f32; 8];
        for value in &mut values {
            *value = tween.tick();
        }
        assert!(values[3] >= 10.0 - EPS);
        assert!(values[5] < values[4]);
    }

    #[test]
    fn tween_pingpong_midpoint() {
        let mut tween = Tween::new(0.0f32, 10.0, 4).with_loop(LoopMode::PingPong);
        let mut value = 0.0;
        for _ in 0..4 {
            value = tween.tick();
        }
        assert!(approx(value, 10.0));
    }

    #[test]
    fn tween_pause_resume() {
        let mut tween = Tween::new(0.0f32, 100.0, 10);
        let value_before_pause = tween.tick();
        tween.pause();
        let paused1 = tween.tick();
        let paused2 = tween.tick();
        assert!(approx(paused1, paused2));
        tween.resume();
        let resumed = tween.tick();
        assert!(resumed > value_before_pause);
    }

    #[test]
    fn tween_reset() {
        let mut tween = Tween::new(0.0f32, 10.0, 4);
        tween.tick();
        tween.tick();
        tween.reset();
        assert!(approx(tween.value(), 0.0));
        assert_eq!(tween.state(), TweenState::Playing);
    }

    #[test]
    fn tween_retarget() {
        let mut tween = Tween::new(0.0f32, 10.0, 10);
        for _ in 0..5 {
            tween.tick();
        }
        let before = tween.value();
        tween.set_target(20.0);
        let after = tween.tick();
        assert!(after > before);
    }

    #[test]
    fn tween_progress() {
        let mut tween = Tween::new(0.0f32, 10.0, 10);
        for _ in 0..5 {
            tween.tick();
        }
        assert!(approx(tween.progress(), 0.5));
    }

    #[test]
    fn sequence_total_duration() {
        let seq = Sequence::new()
            .push(Tween::new(0.0f32, 10.0, 4))
            .push(Tween::new(10.0, 20.0, 6));
        assert_eq!(seq.total_duration(), 10);
    }

    #[test]
    fn sequence_plays_in_order() {
        let mut seq = Sequence::new()
            .push(Tween::new(0.0f32, 10.0, 2))
            .push(Tween::new(10.0, 20.0, 2));

        let v1 = seq.tick();
        let v2 = seq.tick();
        let v3 = seq.tick();

        assert!(v1 <= 10.0);
        assert!(v2 <= 10.0 + EPS);
        assert!(v3 > 10.0);
    }

    #[test]
    fn sequence_value_at_transition() {
        let mut seq = Sequence::new()
            .push(Tween::new(0.0f32, 10.0, 2))
            .push(Tween::new(10.0, 20.0, 2));
        let _ = seq.tick();
        let transition = seq.tick();
        assert!(approx(transition, 10.0));
    }

    #[test]
    fn parallel_finishes_with_longest() {
        let mut parallel = Parallel::new()
            .push(Tween::new(0.0f32, 1.0, 2))
            .push(Tween::new(0.0f32, 1.0, 5));
        for _ in 0..4 {
            parallel.tick();
            assert!(!parallel.is_finished());
        }
        parallel.tick();
        assert!(parallel.is_finished());
    }

    #[test]
    fn parallel_returns_all_values() {
        let mut parallel = Parallel::new()
            .push(Tween::new(0.0f32, 10.0, 4))
            .push(Tween::new(10.0f32, 0.0, 4));
        let values = parallel.tick();
        assert_eq!(values.len(), 2);
    }

    #[test]
    fn stagger_offset() {
        let mut stagger = Stagger::new(2)
            .push(Tween::new(0.0f32, 10.0, 4))
            .push(Tween::new(0.0f32, 10.0, 4));
        let tick1 = stagger.tick();
        assert!(tick1[0] > 0.0);
        assert!(approx(tick1[1], 0.0));

        let _ = stagger.tick();
        let tick3 = stagger.tick();
        assert!(tick3[1] > 0.0);
    }

    #[test]
    fn stagger_total_duration() {
        let stagger = Stagger::new(3)
            .push(Tween::new(0.0f32, 1.0, 2))
            .push(Tween::new(0.0f32, 1.0, 4))
            .push(Tween::new(0.0f32, 1.0, 1));
        assert_eq!(stagger.total_duration(), 7);
    }
}
