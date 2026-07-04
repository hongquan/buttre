use buttre_core::events::{create_event_bus, AppEvent, EventBus};
use std::sync::{Arc, Mutex};

#[test]
fn test_event_bus_creation() {
    let bus = EventBus::new();
    assert_eq!(bus.subscriber_count(), 0);
}

#[test]
fn test_subscribe_and_publish() {
    let bus = EventBus::new();
    let received = Arc::new(Mutex::new(Vec::new()));
    let received_clone = received.clone();

    bus.subscribe(move |event| {
        if let AppEvent::Log { message, .. } = event {
            received_clone.lock().unwrap().push(message.clone());
        }
    });

    bus.publish(AppEvent::info("Test message 1"));
    bus.publish(AppEvent::info("Test message 2"));

    let messages = received.lock().unwrap();
    assert_eq!(messages.len(), 2);
    assert_eq!(messages[0], "Test message 1");
    assert_eq!(messages[1], "Test message 2");
}

#[test]
fn test_multiple_subscribers() {
    let bus = EventBus::new();
    let count1 = Arc::new(Mutex::new(0));
    let count2 = Arc::new(Mutex::new(0));

    let count1_clone = count1.clone();
    bus.subscribe(move |_| {
        *count1_clone.lock().unwrap() += 1;
    });

    let count2_clone = count2.clone();
    bus.subscribe(move |_| {
        *count2_clone.lock().unwrap() += 1;
    });

    bus.publish(AppEvent::info("Test"));

    assert_eq!(*count1.lock().unwrap(), 1);
    assert_eq!(*count2.lock().unwrap(), 1);
}

#[test]
fn test_shared_event_bus() {
    let bus = create_event_bus();
    let bus_clone = bus.clone();

    let received = Arc::new(Mutex::new(false));
    let received_clone = received.clone();

    bus.subscribe(move |_| {
        *received_clone.lock().unwrap() = true;
    });

    // Publish from cloned reference
    bus_clone.publish(AppEvent::info("Test"));

    assert!(*received.lock().unwrap());
}

#[test]
fn test_clear_subscribers() {
    let bus = EventBus::new();

    bus.subscribe(|_| {});
    bus.subscribe(|_| {});
    assert_eq!(bus.subscriber_count(), 2);

    bus.clear();
    assert_eq!(bus.subscriber_count(), 0);
}

#[test]
fn test_handler_panic_isolation() {
    let bus = EventBus::new();
    let received = Arc::new(Mutex::new(false));
    let received_clone = received.clone();

    // First handler panics
    bus.subscribe(|_| {
        panic!("Handler panic!");
    });

    // Second handler should still run
    bus.subscribe(move |_| {
        *received_clone.lock().unwrap() = true;
    });

    bus.publish(AppEvent::info("Test"));

    // Second handler should have run despite first handler panicking
    assert!(*received.lock().unwrap());
}
