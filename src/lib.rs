#![no_std]
extern crate alloc;

pub mod config;
pub mod easing;
pub mod error;
pub mod float;
pub mod keyframes;
pub mod lerp;
pub mod loop_mode;
pub mod observer;
pub mod spring;
pub mod state;
pub mod timeline;
pub mod tween;

pub use config::TweenConfig;
pub use easing::Easing;
pub use error::TweenError;
pub use float::Float;
pub use keyframes::{Keyframe, Keyframes};
pub use lerp::{Angle, Lerp, Rgba};
pub use loop_mode::{LoopMode, PlayDirection};
pub use observer::{NoOpObserver, TweenObserver};
pub use spring::{SpringConfig, SpringTween};
pub use state::TweenState;
pub use timeline::{Timeline, TimelineEntry};
pub use tween::{Parallel, Sequence, Stagger, Tween, TweenId};
