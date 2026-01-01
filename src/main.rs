use std::fs::File;
use std::io::Write;
use rusqlite::{Connection, Result};
use serde_json;
use rand::Rng;

fn main() -> Result<()> {
    let svg = phi12_holographic_fractal(5000, true);
    let mut f = File::create("PHI12_HOLOGRAPHIC_PATENT.svg")?;
    f.write_all(svg.as_bytes())?;

    let conn = Connection::open("holographic_phi12.db")?;
    init_schema(&conn)?;
    let nodes = generate_phi12_nodes(1000);
    store_nodes(&conn, &nodes)?;

    println!("✓ PHI¹² PATENT SVG + HOLOGRAPHIC DB");
    Ok(())
}

fn phi12_holographic_fractal(n: usize, dark: bool) -> String {
    let bg = if dark { " style="background:black;"" } else { "" };
    let mut svg = format!(r#"<svg viewBox="-2 -2 4 4"{bg}>"#, bg=bg);
    
    let phi_b = 0.306348f64;
    let mut theta = 0.0f64;
    
    for i in 0..n {
        let r = (phi_b * theta).exp().min(1.8);
        let x = r * theta.cos();
        let y = r * theta.sin();
        let t = i as f64 / n as f64;
        let g = ((220.0 + 45.0 * t * 1.6180339887).round().clamp(220.0, 255.0) as u8);
        let glow = format!("{:.2}", 0.4 + 0.6 * t + (theta.sin() * 3.0).sin() * 0.3);
        
        svg.push_str(&format!(
            r#"<circle cx="{:.7}" cy="{:.7}" r="0.008" fill="rgb(255,{g},20)" 
            stroke="#ffaa00" stroke-width="0.002" opacity="{glow}"/>"#, x, y, g, glow));
        theta += 1.0 / 1.6180339887;
    }
    
    svg.push_str(r#"
    <defs><filter id="glow" x="-50%" y="-50%" width="200%" height="200%">
    <feGaussianBlur stdDeviation="0.01"/><feMerge><feMergeNode/><feMergeNode in="SourceGraphic"/></feMerge></filter></defs></svg>"#);
    svg
}

fn init_schema(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS phi12_nodes (
            node_id TEXT PRIMARY KEY, phi_coords TEXT, semantic_hash TEXT
        )", [])?;
    Ok(())
}

fn generate_phi12_nodes(count: usize) -> Vec<(String, String, String)> {
    let mut rng = rand::thread_rng();
    (0..count).map(|i| {
        let theta = (i as f64 * 0.6180339887) % (std::f64::consts::PI * 2.0);
        let r = (0.306348 * theta).exp().min(2.0);
        let coords = format!("[{:.3},{:.3},{:.3}]", 
            r * theta.cos(), r * theta.sin(), (theta * 1.618).sin());
        let hash = format!("phi12_{:04}_{:x}", i, 
            (rng.gen::<u64>() ^ (i as u64 * 2654435761)).to_string());
        (format!("phi12_{}", i), coords, hash)
    }).collect()
}

fn store_nodes(conn: &Connection, nodes: &[(String, String, String)]) -> Result<()> {
    let tx = conn.transaction()?;
    for (id, coords, hash) in nodes {
        tx.execute("INSERT OR REPLACE INTO phi12_nodes VALUES (?1,?2,?3)",
            (&id, &coords, &hash))?;
    }
    tx.commit()?;
    Ok(())
}
