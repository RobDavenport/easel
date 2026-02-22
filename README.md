# easel

`easel` is a `no_std` Rust animation primitives library focused on deterministic, tick-based behavior for games and UI runtimes.

## Features

- Deterministic tick timing (`u32`), no floating time deltas.
- Generic interpolation with `Lerp<F>` for scalars, tuples, arrays, `Rgba`, and shortest-path `Angle`.
- Full easing suite (`Easing`) including Penner easings and cubic-bezier.
- Core tweening (`Tween`) with delay, loops, ping-pong, pause/resume, and retargeting.
- Multi-point keyframes (`Keyframes`) with per-segment easing.
- Retargetable spring animation (`SpringTween`) with presets.
- Composition primitives: `Sequence`, `Parallel`, `Stagger`.
- Heterogeneous timing coordinator (`Timeline`) by `TweenId`.

## Crate Layout

- `src/float.rs`: `Float` abstraction with `libm`.
- `src/lerp.rs`: `Lerp`, `Rgba`, `Angle`.
- `src/easing.rs`: easing enum + free easing functions + cubic-bezier solver.
- `src/tween.rs`: `Tween`, `Sequence`, `Parallel`, `Stagger`.
- `src/keyframes.rs`: `Keyframe`, `Keyframes`.
- `src/spring.rs`: `SpringConfig`, `SpringTween`.
- `src/timeline.rs`: `Timeline`, `TimelineEntry`.
- `src/config.rs`: CSS-like easing presets.
- `src/observer.rs`: observer trait + no-op observer.
- `src/error.rs`: error types.

## Quick Example

```rust
use easel::{Easing, Tween};

let mut tween = Tween::new(0.0f32, 100.0, 60).with_easing(Easing::EaseOutCubic);
for _ in 0..60 {
    let _value = tween.tick();
}
assert!(tween.is_finished());
```

## Verification

```bash
cargo test --target x86_64-pc-windows-msvc
cargo build --target wasm32-unknown-unknown --release
```

## GitHub Pages Showcase

This repo includes a browser showcase in `demo-wasm/www/` with four tabs:

1. Easing curves
2. Tween playground
3. Spring physics
4. Timeline editor

### Local Preview

Serve `demo-wasm/www/` with any static server:

```bash
cd demo-wasm/www
python -m http.server 8080
```

### Deployment

GitHub Actions workflow: `.github/workflows/pages.yml`.

1. Enable GitHub Pages in repository settings.
2. Set source to **GitHub Actions**.
3. Push to `main`; workflow deploys `demo-wasm/www` to Pages.
