use buttre_platform::*;

#[test]
fn test_platform_detection() {
    // Exactly one platform should be detected
    let count = [is_windows(), is_macos(), is_linux()]
        .iter()
        .filter(|&&x| x)
        .count();

    assert_eq!(count, 1, "Exactly one platform should be detected");
}

#[test]
fn test_platform_name() {
    let name = platform_name();
    assert!(
        name == "Windows" || name == "macOS" || name == "Linux",
        "Platform name should be Windows, macOS, or Linux"
    );
}

#[test]
fn test_backend_creation() {
    // Test that backend can be created
    let result = Backend::new();
    assert!(result.is_ok(), "Backend creation should succeed");
}
