use easel::{Easing, LoopMode, SpringConfig, SpringTween, Timeline, Tween};
use wasm_bindgen::prelude::*;

/// Lightweight WASM wrapper for browser demos.
///
/// `tick()` advances all internal animations by one simulation step.
/// `render_state()` returns a flat float array:
/// [tween_value, spring_value, active_count, id0, progress0, id1, progress1, ...]
#[wasm_bindgen]
pub struct Demo {
    tween: Tween<f32, f32>,
    spring: SpringTween<f32>,
    timeline: Timeline,
    tween_value: f32,
    spring_value: f32,
    active_flat: Vec<f32>,
}

#[wasm_bindgen]
impl Demo {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut timeline = Timeline::new().with_loop(LoopMode::Infinite);
        let _ = timeline.add(0, 60);
        let _ = timeline.add(20, 45);
        let _ = timeline.add(40, 30);

        Self {
            tween: Tween::new(0.0, 1.0, 60).with_easing(Easing::EaseInOutCubic),
            spring: SpringTween::new(0.0, 1.0, SpringConfig::wobbly()),
            timeline,
            tween_value: 0.0,
            spring_value: 0.0,
            active_flat: Vec::new(),
        }
    }

    pub fn tick(&mut self) {
        self.tween_value = self.tween.tick();
        self.spring_value = self.spring.tick();
        if self.spring.is_at_rest() {
            self.spring.set_target(1.0 - self.spring_value);
        }

        let active = self.timeline.tick::<f32>();
        self.active_flat.clear();
        for (id, progress) in active {
            self.active_flat.push(id.0 as f32);
            self.active_flat.push(progress);
        }
    }

    pub fn reset(&mut self) {
        self.tween.reset();
        self.spring.reset(0.0, 1.0);
        self.timeline.reset();
        self.tween_value = 0.0;
        self.spring_value = 0.0;
        self.active_flat.clear();
    }

    pub fn render_state(&self) -> js_sys::Float32Array {
        let mut state = Vec::with_capacity(3 + self.active_flat.len());
        state.push(self.tween_value);
        state.push(self.spring_value);
        state.push((self.active_flat.len() / 2) as f32);
        state.extend_from_slice(&self.active_flat);
        js_sys::Float32Array::from(state.as_slice())
    }
}

impl Default for Demo {
    fn default() -> Self {
        Self::new()
    }
}
