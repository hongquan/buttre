use buttre_engine::buffer::InputBuffer;

#[test]
fn test_new_buffer() {
    let buffer = InputBuffer::new();
    assert_eq!(buffer.len(), 0);
    assert!(buffer.is_empty());
}

#[test]
fn test_push_pop() {
    let mut buffer = InputBuffer::new();
    buffer.push('a', true);
    buffer.push('B', false);

    assert_eq!(buffer.len(), 2);
    assert_eq!(buffer.pop(), Some(('B', false)));
    assert_eq!(buffer.pop(), Some(('a', true)));
    assert_eq!(buffer.pop(), None);
}

#[test]
fn test_last() {
    let mut buffer = InputBuffer::new();
    assert_eq!(buffer.last(), None);

    buffer.push('a', true);
    assert_eq!(buffer.last(), Some(&'a'));

    buffer.push('b', true);
    assert_eq!(buffer.last(), Some(&'b'));
}

#[test]
fn test_get_set() {
    let mut buffer = InputBuffer::new();
    buffer.push('a', true);
    buffer.push('b', true);

    assert_eq!(buffer.get(0), Some(&'a'));
    assert_eq!(buffer.get(1), Some(&'b'));
    assert_eq!(buffer.get(2), None);

    buffer.set(0, 'â');
    assert_eq!(buffer.get(0), Some(&'â'));
}

#[test]
fn test_clear() {
    let mut buffer = InputBuffer::new();
    buffer.push('a', true);
    buffer.push('b', true);
    buffer.set_last_w_converted(true);

    buffer.clear();

    assert_eq!(buffer.len(), 0);
    assert!(!buffer.last_w_converted());
}

#[test]
fn test_to_string() {
    let mut buffer = InputBuffer::new();
    buffer.push('h', true);
    buffer.push('e', true);
    buffer.push('l', true);
    buffer.push('l', true);
    buffer.push('o', true);

    assert_eq!(buffer.to_string(), "hello");
}

#[test]
fn test_throw_buffer() {
    let mut buffer = InputBuffer::new();

    // Fill buffer beyond capacity (40)
    for i in 0..50 {
        buffer.push(char::from_digit(i % 10, 10).unwrap(), true);
    }

    // After pushing 50 chars:
    // - First 40 chars fill buffer
    // - 41st char triggers throw_buffer, keeps last 20, then adds 41st = 21
    // - 42nd char adds to 22
    // - ...
    // - 50th char -> buffer has 30 chars
    assert_eq!(buffer.len(), 30);
}

#[test]
fn test_chars_from() {
    let mut buffer = InputBuffer::new();
    buffer.push('a', true);
    buffer.push('b', true);
    buffer.push('c', true);

    let chars: String = buffer.chars_from(1).collect();
    assert_eq!(chars, "bc");
}

// ==================== Additional Comprehensive Tests ====================

// === Basic Operations Tests ===

#[test]
fn test_default() {
    let buffer = InputBuffer::default();
    assert_eq!(buffer.len(), 0);
    assert!(buffer.is_empty());
    assert!(!buffer.last_w_converted());
    assert!(!buffer.last_is_escape());
}

#[test]
fn test_multiple_push() {
    let mut buffer = InputBuffer::new();

    // Push multiple characters
    buffer.push('v', true);
    buffer.push('i', true);
    buffer.push('ệ', true);
    buffer.push('t', true);

    assert_eq!(buffer.len(), 4);
    assert_eq!(buffer.to_string(), "việt");
}

#[test]
fn test_push_with_mixed_case_flags() {
    let mut buffer = InputBuffer::new();

    buffer.push('A', false); // Uppercase
    buffer.push('b', true); // Lowercase
    buffer.push('C', false); // Uppercase

    // Pop and verify flags are preserved
    assert_eq!(buffer.pop(), Some(('C', false)));
    assert_eq!(buffer.pop(), Some(('b', true)));
    assert_eq!(buffer.pop(), Some(('A', false)));
}

#[test]
fn test_pop_empty_buffer() {
    let mut buffer = InputBuffer::new();
    assert_eq!(buffer.pop(), None);

    buffer.push('a', true);
    buffer.pop();
    assert_eq!(buffer.pop(), None);
}

// === Get/Set Operations ===

#[test]
fn test_get_all_positions() {
    let mut buffer = InputBuffer::new();
    buffer.push('a', true);
    buffer.push('b', true);
    buffer.push('c', true);

    assert_eq!(buffer.get(0), Some(&'a'));
    assert_eq!(buffer.get(1), Some(&'b'));
    assert_eq!(buffer.get(2), Some(&'c'));
    assert_eq!(buffer.get(3), None);
}

#[test]
fn test_set_multiple_positions() {
    let mut buffer = InputBuffer::new();
    buffer.push('a', true);
    buffer.push('a', true);
    buffer.push('a', true);

    buffer.set(0, 'â');
    buffer.set(2, 'ă');

    assert_eq!(buffer.to_string(), "âaă");
}

#[test]
fn test_set_out_of_bounds() {
    let mut buffer = InputBuffer::new();
    buffer.push('a', true);

    // Should not panic, just ignore
    buffer.set(10, 'x');
    assert_eq!(buffer.len(), 1);
    assert_eq!(buffer.to_string(), "a");
}

#[test]
fn test_set_vietnamese_chars() {
    let mut buffer = InputBuffer::new();
    buffer.push('a', true);
    buffer.push('e', true);
    buffer.push('o', true);

    // Transform to Vietnamese characters
    buffer.set(0, 'â');
    buffer.set(1, 'ê');
    buffer.set(2, 'ô');

    assert_eq!(buffer.to_string(), "âêô");
}

// === Last Character Tests ===

#[test]
fn test_last_after_operations() {
    let mut buffer = InputBuffer::new();

    assert_eq!(buffer.last(), None);

    buffer.push('a', true);
    assert_eq!(buffer.last(), Some(&'a'));

    buffer.push('b', true);
    assert_eq!(buffer.last(), Some(&'b'));

    buffer.pop();
    assert_eq!(buffer.last(), Some(&'a'));

    buffer.clear();
    assert_eq!(buffer.last(), None);
}

#[test]
fn test_last_vietnamese_char() {
    let mut buffer = InputBuffer::new();
    buffer.push('ư', true);
    buffer.push('ơ', true);
    buffer.push('đ', true);

    assert_eq!(buffer.last(), Some(&'đ'));
}

// === Clear Tests ===

#[test]
fn test_clear_resets_all_state() {
    let mut buffer = InputBuffer::new();

    buffer.push('a', true);
    buffer.push('b', false);
    buffer.set_last_w_converted(true);
    buffer.set_last_is_escape(true);

    buffer.clear();

    assert_eq!(buffer.len(), 0);
    assert!(buffer.is_empty());
    assert!(!buffer.last_w_converted());
    assert!(!buffer.last_is_escape());
    assert_eq!(buffer.last(), None);
}

#[test]
fn test_clear_multiple_times() {
    let mut buffer = InputBuffer::new();

    buffer.push('a', true);
    buffer.clear();
    assert_eq!(buffer.len(), 0);

    buffer.push('b', true);
    buffer.clear();
    assert_eq!(buffer.len(), 0);

    // Should still be usable
    buffer.push('c', true);
    assert_eq!(buffer.len(), 1);
}

// === String Conversion Tests ===

#[test]
fn test_to_string_empty() {
    let buffer = InputBuffer::new();
    assert_eq!(buffer.to_string(), "");
}

#[test]
fn test_to_string_single_char() {
    let mut buffer = InputBuffer::new();
    buffer.push('x', true);
    assert_eq!(buffer.to_string(), "x");
}

#[test]
fn test_to_string_vietnamese_word() {
    let mut buffer = InputBuffer::new();
    buffer.push('t', true);
    buffer.push('h', true);
    buffer.push('ư', true);
    buffer.push('ơ', true);
    buffer.push('n', true);
    buffer.push('g', true);

    assert_eq!(buffer.to_string(), "thương");
}

#[test]
fn test_to_string_after_modifications() {
    let mut buffer = InputBuffer::new();
    buffer.push('a', true);
    buffer.push('a', true);

    buffer.set(1, 'â');
    assert_eq!(buffer.to_string(), "aâ");

    buffer.pop();
    assert_eq!(buffer.to_string(), "a");
}

// === Chars From Iterator Tests ===

#[test]
fn test_chars_from_start() {
    let mut buffer = InputBuffer::new();
    buffer.push('h', true);
    buffer.push('e', true);
    buffer.push('l', true);
    buffer.push('l', true);
    buffer.push('o', true);

    let chars: String = buffer.chars_from(0).collect();
    assert_eq!(chars, "hello");
}

#[test]
fn test_chars_from_middle() {
    let mut buffer = InputBuffer::new();
    buffer.push('a', true);
    buffer.push('b', true);
    buffer.push('c', true);
    buffer.push('d', true);

    let chars: String = buffer.chars_from(2).collect();
    assert_eq!(chars, "cd");
}

#[test]
fn test_chars_from_last_position() {
    let mut buffer = InputBuffer::new();
    buffer.push('a', true);
    buffer.push('b', true);
    buffer.push('c', true);

    let chars: String = buffer.chars_from(2).collect();
    assert_eq!(chars, "c");
}

#[test]
fn test_chars_from_end() {
    let mut buffer = InputBuffer::new();
    buffer.push('a', true);
    buffer.push('b', true);

    let chars: String = buffer.chars_from(2).collect();
    assert_eq!(chars, "");
}

// === Buffer Capacity and Throw Tests ===

#[test]
fn test_buffer_at_max_capacity() {
    let mut buffer = InputBuffer::new();

    // Fill exactly to BUFFER_SIZE (40)
    for i in 0..40 {
        buffer.push(char::from_digit(i % 10, 10).unwrap(), true);
    }

    assert_eq!(buffer.len(), 40);
}

#[test]
fn test_throw_buffer_keeps_last_keys() {
    let mut buffer = InputBuffer::new();

    // Fill to 40
    for i in 0..40 {
        buffer.push(char::from_digit(i % 10, 10).unwrap(), true);
    }

    // Add one more (should trigger throw)
    buffer.push('X', true);

    // Should keep last 20 + new one = 21
    assert_eq!(buffer.len(), 21);
    assert_eq!(buffer.last(), Some(&'X'));
}

#[test]
fn test_throw_buffer_multiple_times() {
    let mut buffer = InputBuffer::new();

    // Push 60 characters (will trigger throw twice)
    for i in 0..60 {
        buffer.push(char::from_digit(i % 10, 10).unwrap(), true);
    }

    // After 60 pushes, buffer should have 40 chars
    assert_eq!(buffer.len(), 40);
}

#[test]
fn test_throw_buffer_preserves_recent_chars() {
    let mut buffer = InputBuffer::new();

    // Fill to capacity
    for _ in 0..40 {
        buffer.push('x', true);
    }

    // Push recognizable characters
    buffer.push('A', true);
    buffer.push('B', true);
    buffer.push('C', true);

    // Should have A, B, C in buffer
    assert_eq!(buffer.len(), 23);
    let s = buffer.to_string();
    assert!(s.ends_with("ABC"));
}

// === Flag Management Tests ===

#[test]
fn test_last_w_converted_flag() {
    let mut buffer = InputBuffer::new();

    assert!(!buffer.last_w_converted());

    buffer.set_last_w_converted(true);
    assert!(buffer.last_w_converted());

    buffer.set_last_w_converted(false);
    assert!(!buffer.last_w_converted());
}

#[test]
fn test_last_is_escape_flag() {
    let mut buffer = InputBuffer::new();

    assert!(!buffer.last_is_escape());

    buffer.set_last_is_escape(true);
    assert!(buffer.last_is_escape());

    buffer.set_last_is_escape(false);
    assert!(!buffer.last_is_escape());
}

#[test]
fn test_flags_independent() {
    let mut buffer = InputBuffer::new();

    buffer.set_last_w_converted(true);
    buffer.set_last_is_escape(true);

    assert!(buffer.last_w_converted());
    assert!(buffer.last_is_escape());

    buffer.set_last_w_converted(false);
    assert!(!buffer.last_w_converted());
    assert!(buffer.last_is_escape()); // Should still be true
}

#[test]
fn test_flags_survive_push_pop() {
    let mut buffer = InputBuffer::new();

    buffer.set_last_w_converted(true);
    buffer.push('a', true);
    assert!(buffer.last_w_converted());

    buffer.pop();
    assert!(buffer.last_w_converted());
}

// === Edge Cases ===

#[test]
fn test_empty_buffer_operations() {
    let mut buffer = InputBuffer::new();

    assert_eq!(buffer.len(), 0);
    assert!(buffer.is_empty());
    assert_eq!(buffer.last(), None);
    assert_eq!(buffer.pop(), None);
    assert_eq!(buffer.to_string(), "");
}

#[test]
fn test_single_character_operations() {
    let mut buffer = InputBuffer::new();
    buffer.push('a', true);

    assert_eq!(buffer.len(), 1);
    assert!(!buffer.is_empty());
    assert_eq!(buffer.last(), Some(&'a'));
    assert_eq!(buffer.get(0), Some(&'a'));
    assert_eq!(buffer.to_string(), "a");
}

#[test]
fn test_unicode_characters() {
    let mut buffer = InputBuffer::new();

    // Vietnamese characters
    buffer.push('ă', true);
    buffer.push('â', true);
    buffer.push('đ', true);
    buffer.push('ê', true);
    buffer.push('ô', true);
    buffer.push('ơ', true);
    buffer.push('ư', true);

    assert_eq!(buffer.len(), 7);
    assert_eq!(buffer.to_string(), "ăâđêôơư");
}

#[test]
fn test_tone_marked_characters() {
    let mut buffer = InputBuffer::new();

    // Characters with tone marks
    buffer.push('á', true);
    buffer.push('à', true);
    buffer.push('ả', true);
    buffer.push('ã', true);
    buffer.push('ạ', true);

    assert_eq!(buffer.to_string(), "áàảãạ");
}

#[test]
fn test_complex_vietnamese_word() {
    let mut buffer = InputBuffer::new();

    // Type "trường" character by character
    buffer.push('t', true);
    buffer.push('r', true);
    buffer.push('ư', true);
    buffer.push('ờ', true);
    buffer.push('n', true);
    buffer.push('g', true);

    assert_eq!(buffer.len(), 6);
    assert_eq!(buffer.to_string(), "trường");
    assert_eq!(buffer.last(), Some(&'g'));
}

// === Reuse After Clear ===

#[test]
fn test_reuse_after_clear() {
    let mut buffer = InputBuffer::new();

    // First use
    buffer.push('a', true);
    buffer.push('b', true);
    assert_eq!(buffer.len(), 2);

    // Clear
    buffer.clear();
    assert_eq!(buffer.len(), 0);

    // Reuse
    buffer.push('x', true);
    buffer.push('y', true);
    buffer.push('z', true);
    assert_eq!(buffer.len(), 3);
    assert_eq!(buffer.to_string(), "xyz");
}

// === Lowercase Flags Preservation ===

#[test]
fn test_lowercase_flags_preserved() {
    let mut buffer = InputBuffer::new();

    buffer.push('A', false); // Uppercase
    buffer.push('b', true); // Lowercase
    buffer.push('C', false); // Uppercase
    buffer.push('d', true); // Lowercase

    // Pop in reverse order and verify flags
    let (ch4, flag4) = buffer.pop().unwrap();
    assert_eq!(ch4, 'd');
    assert!(flag4);

    let (ch3, flag3) = buffer.pop().unwrap();
    assert_eq!(ch3, 'C');
    assert!(!flag3);

    let (ch2, flag2) = buffer.pop().unwrap();
    assert_eq!(ch2, 'b');
    assert!(flag2);

    let (ch1, flag1) = buffer.pop().unwrap();
    assert_eq!(ch1, 'A');
    assert!(!flag1);
}

#[test]
fn test_mixed_operations_sequence() {
    let mut buffer = InputBuffer::new();

    // Complex sequence of operations
    buffer.push('h', true);
    buffer.push('e', true);
    buffer.push('l', true);
    assert_eq!(buffer.len(), 3);

    buffer.set(1, 'ê');
    assert_eq!(buffer.to_string(), "hêl");

    buffer.push('l', true);
    buffer.push('o', true);
    assert_eq!(buffer.to_string(), "hêllo");

    buffer.pop();
    assert_eq!(buffer.to_string(), "hêll");

    buffer.set_last_w_converted(true);
    assert!(buffer.last_w_converted());

    buffer.clear();
    assert_eq!(buffer.len(), 0);
    assert!(!buffer.last_w_converted());
}
