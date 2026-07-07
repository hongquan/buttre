use buttre_platform::shared::config_watcher::ConfigWatcher;
use std::fs;
use std::thread;
use std::time::Duration;

#[test]
fn test_watcher_creation() {
    let temp_dir = std::env::temp_dir().join("buttre_test_keyboards");
    let _ = fs::create_dir_all(&temp_dir);

    let watcher = ConfigWatcher::new(temp_dir.clone());
    assert!(watcher.is_ok());

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_event_detection() {
    let temp_dir = std::env::temp_dir().join("buttre_test_keyboards_2");
    let _ = fs::create_dir_all(&temp_dir);

    let watcher = ConfigWatcher::new(temp_dir.clone()).unwrap();

    // Create a test config file
    let test_file = temp_dir.join("test.toml");
    fs::write(&test_file, "# test").unwrap();

    // Wait for event to be processed
    thread::sleep(Duration::from_secs(3));

    // Check if event was received
    let event = watcher.try_recv();
    assert!(event.is_some());

    // Cleanup
    let _ = fs::remove_file(&test_file);
    let _ = fs::remove_dir_all(&temp_dir);
}
