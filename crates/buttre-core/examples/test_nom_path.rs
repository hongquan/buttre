// Quick test for get_nom_db_path()
fn main() {
    println!("Testing get_nom_db_path()...\n");
    println!("Current exe: {:?}", std::env::current_exe());
    println!("Current dir: {:?}\n", std::env::current_dir());
    
    let path = buttre_core::vietnamese::get_nom_db_path();
    
    match path {
        Some(p) => {
            println!("\n✓ Found dictionary at: {:?}", p);
            println!("  File exists: {}", p.exists());
            if let Ok(metadata) = std::fs::metadata(&p) {
                println!("  File size: {} bytes ({:.2} MB)", metadata.len(), metadata.len() as f64 / 1_048_576.0);
            }
        }
        None => {
            println!("\n✗ Dictionary not found!");
            println!("\nSearched in:");
            if let Ok(exe) = std::env::current_exe() {
                if let Some(dir) = exe.parent() {
                    println!("  1. {:?}", dir.join("buttre_nom.db"));
                    println!("  2. {:?}", dir.join("resources/nom/buttre_nom.db"));
                }
            }
            println!("  3. {:?}", std::path::PathBuf::from("buttre_nom.db"));
            if let Some(data_dir) = dirs::data_local_dir() {
                println!("  4. {:?}", data_dir.join("buttre/buttre_nom.db"));
            }
        }
    }
}
