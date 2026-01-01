pub fn phi12_fractal(iter: u32) -> f64 {
    let mut sum = 0.0f64;
    let phi: f64 = 1.6180339887;
    for i in 0..iter {
        let angle = (i as f64 * 0.618) * std::f64::consts::PI * 2.0;
        sum += angle.sin() * phi.powf((i as i32 % 12) as f64);
    }
    sum / iter as f64
}
