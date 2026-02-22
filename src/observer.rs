use crate::tween::TweenId;

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

#[cfg(test)]
mod tests {
    use crate::observer::{NoOpObserver, TweenObserver};
    use crate::tween::TweenId;

    #[test]
    fn observer_noop_compiles() {
        let mut observer = NoOpObserver;
        observer.on_start(TweenId(1));
        observer.on_pause(TweenId(1));
        observer.on_resume(TweenId(1));
        observer.on_loop(TweenId(1), 2);
        observer.on_complete(TweenId(1));
    }
}
