use std::fs::File;
use std::io::Write;

fn main() {
    let svg = phi12_svg_stable(2500, true);
    let mut file = File::create("phi12_patent_dark.svg").unwrap();  // Same filename
    file.write_all(svg.as_bytes()).unwrap();
}

fn phi12_svg_stable(iter: usize, dark: bool) -> String {
    let bg = if dark { r#" style="background:black;""# } else { "" };
    let mut svg = format!(r#"<svg viewBox="-1.5 -1.5 3 3" xmlns="http://www.w3.org/2000/svg"{bg}>"#, bg=bg);
    
    for i in 0..iter {
        let t = i as f64 / iter as f64;
        let angle = t * 10.0 * std::f64::consts::TAU;
        let r = 0.8 * t.sqrt() * (1.0 + 0.3 * (t * 8.0).sin());
        let x = r * angle.cos();
        let y = r * angle.sin();
        let g = ((200.0 + 55.0 * t) as u32) as u8;  // 200→255 golden fade
        let opacity = 0.4 + 0.6 * t;
        
        // Fixed: unique args, no unused vars
        svg.push_str(&format!(
            r#"<circle cx="{:.6}" cy="{:.6}" r="0.008" fill="rgb(255,{g},20)" 
            stroke="rgb(255,165,0)" stroke-width="0.002" opacity="{:.3}"/>"#,
            x, y, g, opacity  // 4 args → 4 placeholders
        ));
    }
    svg.push_str("</svg>");
    svg
}
