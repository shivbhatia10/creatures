pub fn cheap_normal(x: f32) -> f32 {
    // Like a normal distribution but less expensive
    1.0 / (1.0 + x * x)
}
