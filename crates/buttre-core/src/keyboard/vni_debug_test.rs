#[cfg(test)]
mod vni_debug_tests {
    use crate::KeyboardBuilder;
    use crate::Action;
    
    #[test]
    fn test_vni_a6_transformation() {
        let mut kb = KeyboardBuilder::vni().expect("Failed to create VNI keyboard");
        
        // Test: a + 6 should produce â
        println!("Processing 'a'...");
        let action1 = kb.process('a').expect("Failed to process 'a'");
        println!("  Result: {:?}", action1);
        
        println!("Processing '6'...");
        let action2 = kb.process('6').expect("Failed to process '6'");
        println!("  Result: {:?}", action2);
        
        // Expected: Replace { backspace_count: 1, text: "â" }
        assert!(!action2.is_empty(), "Should have at least one action");
        match &action2[0] {
            Action::Replace { backspace_count, text } => {
                assert_eq!(*backspace_count, 1, "Should backspace 1 character");
                assert_eq!(text, "â", "Should replace with â");
            }
            _ => panic!("Expected Replace action, got {:?}", action2),
        }
    }
    
    #[test]
    fn test_vni_transformations() {
        let mut kb = KeyboardBuilder::vni().expect("Failed to create VNI keyboard");
        
        let test_cases = vec![
            (vec!['a', '6'], "â"),
            (vec!['a', '8'], "ă"),  // VNI: 8 for breve (ă)
            (vec!['e', '6'], "ê"),
            (vec!['o', '6'], "ô"),
            (vec!['o', '7'], "ơ"),
            (vec!['u', '7'], "ư"),
        ];
        
        for (keys, expected) in test_cases {
            kb.reset();
            let mut last_action = None;
            
            for key in keys.iter() {
                last_action = Some(kb.process(*key).expect("Failed to process key"));
            }
            
            if let Some(actions) = last_action {
                assert!(!actions.is_empty(), "Should have at least one action for {:?}", keys);
                if let Action::Replace { text, .. } = &actions[0] {
                    assert_eq!(text, expected, "Failed for keys: {:?}", keys);
                } else {
                    panic!("Expected Replace action for {:?}, got {:?}", keys, actions);
                }
            } else {
                panic!("No action for {:?}", keys);
            }
        }
    }
}
