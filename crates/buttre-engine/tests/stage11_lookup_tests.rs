use buttre_engine::pipeline::dictionary::SimpleDictionary;
use buttre_engine::pipeline::stages::stage11_lookup::LookupStage;
use buttre_engine::pipeline::{
    CandidateType, PipelineConfig, PipelineStage, StageResult, TypingContext,
};
use std::sync::Arc;

#[test]
fn test_disabled_lookup() {
    let stage = LookupStage::new(false);
    let mut ctx = TypingContext::new();
    ctx.syllable_buffer = "nguoi".to_string();

    let result = stage.process(&mut ctx, 'i');

    assert_eq!(result, StageResult::Continue);
    assert!(ctx.candidates.is_empty());
    assert!(!ctx.showing_candidates);
}

#[test]
fn test_enabled_lookup_no_dictionary() {
    let stage = LookupStage::new(true);
    let mut ctx = TypingContext::new();
    ctx.syllable_buffer = "nguoi".to_string();

    let result = stage.process(&mut ctx, 'i');

    // Should continue but no candidates (no dictionary)
    assert_eq!(result, StageResult::Continue);
    assert!(ctx.candidates.is_empty());
}

#[test]
fn test_lookup_with_dictionary() {
    // Create dictionary with test data
    let mut dict = SimpleDictionary::new();
    dict.add("nguoi", "người", CandidateType::Vietnamese, 1.0);
    dict.add("nguoi", "𠊛", CandidateType::Nom, 0.5);

    let stage = LookupStage::with_dictionary(Arc::new(dict));
    let mut ctx = TypingContext::new();
    ctx.syllable_buffer = "nguoi".to_string();

    let result = stage.process(&mut ctx, 'i');

    assert_eq!(result, StageResult::Continue);
    assert_eq!(ctx.candidates.len(), 2);
    assert!(ctx.showing_candidates);
    assert_eq!(ctx.candidates[0].text, "người");
    assert_eq!(ctx.candidates[1].text, "𠊛");
}

#[test]
fn test_lookup_no_matches() {
    let dict = SimpleDictionary::new();
    let stage = LookupStage::with_dictionary(Arc::new(dict));
    let mut ctx = TypingContext::new();
    ctx.syllable_buffer = "xyz".to_string();

    let result = stage.process(&mut ctx, 'z');

    assert_eq!(result, StageResult::Continue);
    assert!(ctx.candidates.is_empty());
    assert!(!ctx.showing_candidates);
}

#[test]
fn test_stage_name() {
    let stage = LookupStage::new(false);
    assert_eq!(stage.name(), "LookupStage");
}

#[test]
fn test_from_config() {
    let mut config = PipelineConfig::new("test");
    config.enable_lookup = true;

    let stage = LookupStage::from_config(&config);

    assert!(stage.enabled);
    assert!(stage.dictionary.is_none());
}

#[test]
fn test_empty_buffer() {
    let mut dict = SimpleDictionary::new();
    dict.add("test", "test", CandidateType::Vietnamese, 1.0);

    let stage = LookupStage::with_dictionary(Arc::new(dict));
    let mut ctx = TypingContext::new();

    let result = stage.process(&mut ctx, 'a');

    assert_eq!(result, StageResult::Continue);
    assert!(ctx.candidates.is_empty());
}

#[test]
fn test_multiple_process_calls() {
    let mut dict = SimpleDictionary::new();
    dict.add("a", "á", CandidateType::Vietnamese, 1.0);
    dict.add("ab", "áb", CandidateType::Vietnamese, 1.0);

    let stage = LookupStage::with_dictionary(Arc::new(dict));
    let mut ctx = TypingContext::new();

    // First call - "a"
    ctx.syllable_buffer = "a".to_string();
    assert_eq!(stage.process(&mut ctx, 'a'), StageResult::Continue);
    assert_eq!(ctx.candidates.len(), 1);

    // Second call - "ab"
    ctx.syllable_buffer = "ab".to_string();
    assert_eq!(stage.process(&mut ctx, 'b'), StageResult::Continue);
    assert_eq!(ctx.candidates.len(), 1);

    // Third call - "abc" (no match)
    ctx.syllable_buffer = "abc".to_string();
    assert_eq!(stage.process(&mut ctx, 'c'), StageResult::Continue);
    assert!(ctx.candidates.is_empty());
}

#[test]
fn test_candidate_ranking() {
    let mut dict = SimpleDictionary::new();
    // Add candidates with different scores
    dict.add("troi", "trời", CandidateType::Vietnamese, 1.0);
    dict.add("troi", "𡗶", CandidateType::Nom, 0.8);
    dict.add("troi", "troi", CandidateType::English, 0.3);

    let stage = LookupStage::with_dictionary(Arc::new(dict));
    let mut ctx = TypingContext::new();
    ctx.syllable_buffer = "troi".to_string();

    stage.process(&mut ctx, 'i');

    assert_eq!(ctx.candidates.len(), 3);
    // Verify candidates are stored (ranking is done by dictionary)
    assert_eq!(ctx.candidates[0].text, "trời");
    assert_eq!(ctx.candidates[1].text, "𡗶");
    assert_eq!(ctx.candidates[2].text, "troi");
}
