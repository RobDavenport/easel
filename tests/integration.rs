use easel::{Easing, Rgba, SpringConfig, SpringTween, Tween};

#[test]
fn integration_ui_slide_in() {
    let mut tween = Tween::new(0.0f32, 300.0, 60).with_easing(Easing::EaseOutCubic);
    for _ in 0..60 {
        tween.tick();
    }
    assert!(tween.is_finished());
    assert!((tween.value() - 300.0).abs() < 1e-3);
}

#[test]
fn integration_color_fade() {
    let from = Rgba::new(1.0f32, 0.0, 0.0, 1.0);
    let to = Rgba::new(0.0f32, 0.0, 1.0, 1.0);
    let mut tween = Tween::new(from, to, 30).with_easing(Easing::EaseInOutSine);
    for _ in 0..30 {
        tween.tick();
    }
    let v = tween.value();
    assert!(v.b > 0.99);
    assert!(v.r < 0.01);
}

#[test]
fn integration_camera_spring() {
    let mut spring = SpringTween::new(0.0f32, 10.0, SpringConfig::stiff());
    for _ in 0..240 {
        spring.tick();
    }
    assert!((spring.value() - 10.0).abs() < 0.1);

    spring.set_target(-5.0);
    let before = spring.value();
    let after = spring.tick();
    assert!(after < before);
}
