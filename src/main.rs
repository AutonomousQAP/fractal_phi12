use std::fs::File;
use std::io::Write;

fn main() {
    let svg = phi12_svg_dark(2500);
    let mut file = File::create("phi12_patent_dark.svg").unwrap();
    file.write_all(svg.as_bytes()).unwrap();
    println!("✓ phi12_patent_dark.svg (black bg, no opacity dupes)");
}

pub fn phi12_svg_dark(iter: usize) -> String {
    let mut svg = String::new();
    svg.push_str(r#"<svg viewBox="-5 -5 10 10" xmlns="http://www.w3.org/2000/svg" style="background:black;">"#);
    let phi = 1.6180339887f64;
    for i in 0..iter {
        let angle = i as f64 * 0.618034 * std::f64::consts::PI * 2.0;
        let r = phi.powf(i as f64 * 0.06) * 0.2;
        let x = r * angle.cos();
        let y = r * angle.sin();
        let alpha = (i as f64 / iter as f64 * 0.7 + 0.3) * 255.0;
        let circle = format!(
            r#"<circle cx="{:.3}" cy="{:.3}" r="0.035" fill="rgb(255,{:.0},0)" stroke="rgb(255,165,0)" stroke-width="0.008"/>"#,
            x, y, alpha
        );
        svg.push_str(&circle);
    }
    svg.push_str("</svg>");
    svg
}
