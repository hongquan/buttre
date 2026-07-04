//! Test cases for tone placement bugs
//! Bug 1: huyeenf -> huỳên (should be huyền)
//! Bug 2: thuowr -> thửơ (should be thuở)

use buttre_engine::pipeline::presets::telex_config;
use buttre_engine::pipeline::PipelineExecutor;

fn test_telex(input: &str) -> String {
    let config = telex_config();
    let mut executor = PipelineExecutor::new(config);

    for ch in input.chars() {
        executor.process(ch);
    }

    executor.syllable().to_string()
}

#[test]
fn test_huyeenf_should_be_huyen() {
    // huyeenf: h-u-y-e-e-n-f
    // ee -> ê, so huyên + f (grave tone)
    // Expected: huyền (tone on ê because ê is super vowel)
    let result = test_telex("huyeenf");
    println!("huyeenf -> '{}'", result);
    assert_eq!(result, "huyền", "Tone should be on ê (super vowel), not y");
}

#[test]
fn test_thuowr_should_be_thuo() {
    // thuowr: th-u-o-w-r
    // ow -> ơ, then w after uo makes ươ
    // Expected: thuở (tone on ơ because ơ is super vowel)
    let result = test_telex("thuowr");
    println!("thuowr -> '{}'", result);
    assert_eq!(result, "thuở", "Tone should be on ơ (super vowel), not ư");
}

#[test]
fn test_simple_ee_tone() {
    // heenf: ee -> ê, f is grave
    let result = test_telex("heenf");
    println!("heenf -> '{}'", result);
    assert_eq!(result, "hền", "Tone should be on ê");
}

#[test]
fn test_simple_ow_tone() {
    // thowr: ow -> ơ, r is hook
    let result = test_telex("thowr");
    println!("thowr -> '{}'", result);
    assert_eq!(result, "thở", "Tone should be on ơ");
}

#[test]
fn test_uow_combination() {
    // tuowr: uo + w -> ươ, r is hook
    // ơ is super vowel, should receive tone
    // Result: tuở (ư + ở where ở = ơ with hook tone)
    let result = test_telex("tuowr");
    println!("tuowr -> '{}'", result);
    // Note: "tuở" means the tone is correctly on ơ (which becomes ở)
    // The result shows ư (without tone) + ở (ơ with hook tone)
    assert_eq!(result, "tuở", "Tone should be on ơ (super vowel)");
}
