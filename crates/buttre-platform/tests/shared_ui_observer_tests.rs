use buttre_platform::shared::observers::ui_observer::{UIObserver, UICallback};
use buttre_core::StateObserver;
use std::sync::{Arc, Mutex};

struct MockUICallback {
    last_method: Mutex<String>,
    last_enabled: Mutex<bool>,
}

impl MockUICallback {
    fn new() -> Self {
        Self {
            last_method: Mutex::new(String::new()),
            last_enabled: Mutex::new(false),
        }
    }
}

impl UICallback for MockUICallback {
    fn update_menu_checkmarks(&self, method: &str) {
        *self.last_method.lock().unwrap() = method.to_string();
    }
    
    fn update_tray_icon(&self, method: &str, enabled: bool) {
        *self.last_method.lock().unwrap() = method.to_string();
        *self.last_enabled.lock().unwrap() = enabled;
    }
}

#[test]
fn test_ui_observer() {
    let callback = Arc::new(MockUICallback::new());
    let observer = UIObserver::new(callback.clone());
    
    // Simulate state change
    observer.on_method_changed("telex", true);
    
    // Verify callback was called
    assert_eq!(*callback.last_method.lock().unwrap(), "telex");
    assert_eq!(*callback.last_enabled.lock().unwrap(), true);
}
