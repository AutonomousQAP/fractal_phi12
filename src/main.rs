use std::fs::File;
use std::io::Write;

fn main() {
    let svg = phi12_fractal(3000, true);
    let mut f = File::create("phi12_patent_dark.svg").unwrap();
    f.write_all(svg.as_bytes()).unwrap();
    println!("✓ phi12_patent_dark.svg (stable φ¹² spiral)");
}

fn phi12_fractal(n: usize, dark: bool) -> String {
    let bg = if dark { " style="background:#000;"" } else { "" };
    let mut s = format!(r#"<svg viewBox="-1.4 -1.4 2.8 2.8" xmlns="http://www.w3.org/2000/svg"{bg}>"#, bg=bg);
    
    let phi_b = 0.306348;  // ln(φ)/(2π) for exact golden spiral
    let mut theta = 0.0;
    
    for i in 0..n {
        let r = (phi_b * theta).exp().min(1.2);
        let x = r * theta.cos();
        let y = r * theta.sin();
        let t = i as f64 / n as f64;
        let g = ((220.0 + 35.0 * t * 1.618).round() as u8).clamp(220, 255);
        let opacity = 0.25 + 0.75 * t;
        
        s.push_str(&format!(
            r#"<circle cx="{:.7}" cy="{:.7}" r="0.01" fill="rgb(255,{g},25)" 
            stroke="#ffaa00" stroke-width="0.0025" opacity="{:.2}"/>"#,
            x, y, g, opacity
        ));
        theta += 0.618034;  // φ⁻¹ step
    }
    s.push_str("</svg>");
    s
}
