#![allow(dead_code)]

pub(crate) fn lerp(t: f32, from: f32, to: f32) -> f32 {
    (to - from) * t + from
}

pub(crate) fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub(crate) fn map(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    (value - from_min) * (to_max - to_min) / (from_max - from_min) + to_min
}

pub(crate) fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

pub(crate) fn smootherstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

/// x elem [0, 1]
/// count elem [1, inf]
pub(crate) fn staircase(x: f32, count: f32) -> f32 {
    (x * count).floor() / count
}