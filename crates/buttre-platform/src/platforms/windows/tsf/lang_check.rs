// Helper function to check Vietnamese language installation
// Placed in separate file to avoid escape character issues

use std::process::Command;

/// Check if Vietnamese language is installed in Windows
/// Returns true if user has added Vietnamese to their language list
pub fn is_vietnamese_language_installed() -> bool {
    // Use PowerShell to check if Vietnamese language is installed
    let output = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            "(Get-WinUserLanguageList | Where-Object { $_.LanguageTag -like 'vi*' }).Count -gt 0"
        ])
        .output();
    
    match output {
        Ok(output) => {
            let result = String::from_utf8_lossy(&output.stdout);
            result.trim() == "True"
        }
        Err(_) => {
            // If PowerShell command fails, assume Vietnamese is not installed
            false
        }
    }
}
