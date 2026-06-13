// Test NomDictionary loading
use buttre_engine::pipeline::nom_dictionary::NomDictionary;

fn main() {
    println!("Testing NomDictionary::open()...\n");
    
    // Get dictionary path
    let path = buttre_core::vietnamese::get_nom_db_path();
    
    match path {
        Some(p) => {
            println!("Dictionary path: {:?}", p);
            println!("File exists: {}", p.exists());
            
            // Try to open
            println!("\nAttempting to open dictionary...");
            match NomDictionary::open(p.clone()) {
                Ok(_dict) => {
                    println!("✓ Successfully opened Nôm dictionary!");
                }
                Err(e) => {
                    println!("✗ Failed to open dictionary: {}", e);
                    println!("   Error details: {:?}", e);
                }
            }
        }
        None => {
            println!("✗ Dictionary file not found!");
        }
    }
}
