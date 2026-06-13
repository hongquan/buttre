// Commented out: This test accesses private method find_main_vowel
// TODO: Either make find_main_vowel public or test through public API
/*
use buttre_engine::pipeline::stages::stage5_tone::ToneStage;
use buttre_engine::pipeline::{PipelineStage, TypingContext};

#[test]
fn test_find_main_vowel_uong() {
    let tone_stage = ToneStage::new(std::collections::HashMap::new());
    
    // Test "trương" - should find 'ơ' (position 3 in bytes, but we need char position)
    let syllable = "trương";
    let vowel_pos = tone_stage.find_main_vowel(syllable);
    
    // Get char positions
    let chars: Vec<(usize, char)> = syllable.char_indices().collect();
    println!("Syllable: {}", syllable);
    println!("Chars: {:?}", chars);
    println!("Vowel pos (byte index): {:?}", vowel_pos);
    
    // Find which char this byte index corresponds to
    if let Some(pos) = vowel_pos {
        let char_at_pos = syllable.chars().nth(chars.iter().position(|(idx, _)| *idx == pos).unwrap());
        println!("Char at vowel pos: {:?}", char_at_pos);
        assert_eq!(char_at_pos, Some('ơ'), "Tone should be on 'ơ' for 'trương'");
    }
}
*/

