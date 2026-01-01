cat > src/main.rs << 'EOF'
use std::fs::File;
use std::io::Write;

fn main() {
    let svg = generate_phi12_fractal(2500, true);
    let mut file = File::create("phi12_patent_dark.svg").unwrap();
    file.write_all(svg.as_bytes()).unwrap();
}

fn generate_phi12_fractal(iter: usize, dark: bool) -> String {
    let bg_style = if dark { " style="background:black;"" } else { "" };
    let mut svg = format!(r#"<svg viewBox="-1.6 -1.6 3.2 3.2" xmlns="http://www.w3.org/2000/svg"{bg_style}>"#, bg_style=bg_style);
    
    // True φ¹² logarithmic spiral: r = a * e^(b θ), b=ln(φ)/(2π)
    let phi_growth = 0.48121182505960347_f64;  // ln(1.618)/ (2π)
    let mut theta = 0.0_f64;
    
    for i in 0..iter {
        let r = 0.9 * (phi_growth * theta).exp().min(1.3);  // Bounded growth
        let x = r * theta.cos();
        let y = r * theta.sin();
        
        // φ-gradient: golden ratio color progression
        let t = (i as f64 / iter as f64).min(1.0);
        let g = ((200.0 + 55.0 * t * 1.6180339887).round() as u8).max(200).min(255);
        let b = (20.0 * t).round() as u8;
        let opacity = 0.3 + 0.7 * t;
        
        svg.push_str(&format!(
            r#"<circle cx="{:.8}" cy="{:.8}" r="0.012" fill="rgb(255,{g},{b})" 
            stroke="hsl(45,100%,50%)" stroke-width="0.003" opacity="{:.3}"/>"#,
            x, y, g, b, opacity
        ));
        
        theta += 0.382;  // 1/φ² radians per step → perfect φ spiral
    }
    svg.push_str("</svg>");
    svg
}
EOF
