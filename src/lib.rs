use rusqlite::Connection;

pub fn phi_spiral() -> String {
    let mut s = "<svg viewBox='-2 -2 4 4' style='background:black;'>".to_string();
    let mut t:f64 = 0.0;
    for i in 0..5000 {
        let r = (0.306 * t).exp().min(1.8);
        let x = r * t.cos();
        let y = r * t.sin();
        let g = (220.0 + 45.0 * (i as f64/5000.0) * 1.618).round().clamp(220.0,255.0) as u8;
        let o = format!("{:.2}", 0.4 + 0.6*(i as f64/5000.0));
        s.push_str(&format!("<circle cx='{:.6}' cy='{:.6}' r='0.008' fill='rgb(255,{},20)' stroke='#ffaa00' stroke-width='0.002' opacity='{}'/>", x,y,g,o));
        t += 0.618;
    }
    s.push_str("<filter id='g'><feGaussianBlur stdDeviation='0.01'/><feMerge><feMergeNode/><feMergeNode in='SourceGraphic'/></feMerge></filter></svg>");
    s
}
