use alloc::vec::Vec;

use crate::float::Float;
use crate::loop_mode::LoopMode;
use crate::state::TweenState;
use crate::tween::TweenId;

/// A timing entry in the timeline.
#[derive(Clone, Debug)]
pub struct TimelineEntry {
    pub id: TweenId,
    pub start_tick: u32,
    pub duration: u32,
}

/// Heterogeneous animation timeline.
#[derive(Clone, Debug)]
pub struct Timeline {
    entries: Vec<TimelineEntry>,
    next_id: u32,
    elapsed: u32,
    state: TweenState,
    loop_mode: LoopMode,
    loops_completed: u32,
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            next_id: 0,
            elapsed: 0,
            state: TweenState::Playing,
            loop_mode: LoopMode::Once,
            loops_completed: 0,
        }
    }

    /// Add an entry. Returns the TweenId for lookup.
    pub fn add(&mut self, start_tick: u32, duration: u32) -> TweenId {
        let id = TweenId(self.next_id);
        self.next_id = self.next_id.saturating_add(1);
        self.entries.push(TimelineEntry {
            id,
            start_tick,
            duration,
        });
        id
    }

    /// Advance by one tick. Returns (TweenId, progress) for active entries.
    pub fn tick<F: Float>(&mut self) -> Vec<(TweenId, F)> {
        if self.state != TweenState::Playing {
            return self.active_entries(self.elapsed);
        }

        let total = self.total_duration();
        if total == 0 {
            self.state = TweenState::Finished;
            return Vec::new();
        }

        if self.elapsed < total {
            self.elapsed += 1;
        }

        let active = self.active_entries(self.elapsed);

        if self.elapsed >= total {
            self.on_iteration_complete();
        }

        active
    }

    /// Total duration (end tick of last entry).
    pub fn total_duration(&self) -> u32 {
        self.entries
            .iter()
            .map(|e| e.start_tick.saturating_add(e.duration))
            .max()
            .unwrap_or(0)
    }

    /// Seek to a specific tick.
    pub fn seek(&mut self, tick: u32) {
        self.elapsed = tick;
        if self.elapsed < self.total_duration() {
            self.state = TweenState::Playing;
        }
    }

    pub fn is_finished(&self) -> bool {
        self.state == TweenState::Finished
    }

    pub fn with_loop(mut self, mode: LoopMode) -> Self {
        self.loop_mode = mode;
        self
    }

    pub fn reset(&mut self) {
        self.elapsed = 0;
        self.loops_completed = 0;
        self.state = TweenState::Playing;
    }

    fn active_entries<F: Float>(&self, tick: u32) -> Vec<(TweenId, F)> {
        let mut active = Vec::new();
        for entry in &self.entries {
            if entry.duration == 0 {
                if tick == entry.start_tick {
                    active.push((entry.id, F::one()));
                }
                continue;
            }

            let end_tick = entry.start_tick.saturating_add(entry.duration);
            if tick >= entry.start_tick && tick <= end_tick {
                let local_elapsed = tick.saturating_sub(entry.start_tick);
                let progress = F::from_f32(local_elapsed as f32 / entry.duration as f32)
                    .clamp(F::zero(), F::one());
                active.push((entry.id, progress));
            }
        }
        active
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

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::loop_mode::LoopMode;
    use crate::timeline::Timeline;

    const EPS: f32 = 1e-4;

    #[test]
    fn timeline_active_entries() {
        let mut timeline = Timeline::new();
        let id_a = timeline.add(0, 10);
        let id_b = timeline.add(5, 10);

        for _ in 0..4 {
            let active = timeline.tick::<f32>();
            assert!(active.iter().any(|(id, _)| *id == id_a));
            assert!(!active.iter().any(|(id, _)| *id == id_b));
        }

        let active = timeline.tick::<f32>();
        assert!(active.iter().any(|(id, _)| *id == id_a));
        assert!(active.iter().any(|(id, _)| *id == id_b));
    }

    #[test]
    fn timeline_progress() {
        let mut timeline = Timeline::new();
        let id = timeline.add(10, 20);
        timeline.seek(19);
        let active = timeline.tick::<f32>();
        let (_, progress) = active
            .iter()
            .find(|(entry_id, _)| *entry_id == id)
            .copied()
            .expect("entry should be active");
        assert!((progress - 0.5).abs() < EPS);
    }

    #[test]
    fn timeline_seek() {
        let mut timeline = Timeline::new();
        let id = timeline.add(0, 10);
        timeline.seek(7);
        let active = timeline.tick::<f32>();
        let (_, progress) = active
            .iter()
            .find(|(entry_id, _)| *entry_id == id)
            .copied()
            .expect("entry should be active");
        assert!((progress - 0.8).abs() < EPS);
    }

    #[test]
    fn timeline_loop() {
        let mut timeline = Timeline::new().with_loop(LoopMode::Infinite);
        timeline.add(0, 3);
        for _ in 0..100 {
            let _ = timeline.tick::<f32>();
        }
        assert!(!timeline.is_finished());
    }
}
