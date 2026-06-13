use buttre_engine::vowel::positioning::{find_tone_position, find_tone_position_from_table, is_super_vowel, TonePositioningMode};
use buttre_engine::vowel::cluster::{VowelCluster, ClusterType};
// Need to expose find_phonology_position and find_free_position or use public interface
// The tests test private functions. I should test through public interface where possible, or expose them.
// find_tone_position delegates to them, so public interface testing is preferred.
// However, find_phonology_position and find_free_position are tested directly.
use buttre_engine::vowel::positioning::{find_phonology_position, find_free_position}; 

fn make_cluster(start: usize, vowels: Vec<char>, cluster_type: ClusterType) -> VowelCluster {
    VowelCluster {
        start_pos: start,
        end_pos: start + vowels.len(),
        vowels,
        cluster_type,
    }
}

#[test]
fn test_is_super_vowel() {
    assert!(is_super_vowel('ă'));
    assert!(is_super_vowel('â'));
    assert!(is_super_vowel('ê'));
    assert!(is_super_vowel('ô'));
    assert!(is_super_vowel('ơ'));
    
    assert!(is_super_vowel('Ă'));
    assert!(is_super_vowel('Â'));
    
    assert!(!is_super_vowel('a'));
    assert!(!is_super_vowel('e'));
    assert!(!is_super_vowel('i'));
}

#[test]
fn test_find_phonology_position_single_vowel() {
    let cluster = make_cluster(1, vec!['a'], ClusterType::Single);
    let pos = find_phonology_position("bat", &cluster);
    assert_eq!(pos, Some(1));
}

#[test]
fn test_find_phonology_position_super_vowel() {
    // "tên" - cluster is "ê"
    let cluster = make_cluster(1, vec!['ê'], ClusterType::Single);
    let pos = find_phonology_position("tên", &cluster);
    assert_eq!(pos, Some(1));
    
    // "trường" - cluster is "ươ", tone on ơ
    let cluster = make_cluster(2, vec!['ư', 'ơ'], ClusterType::CompoundUO);
    let pos = find_phonology_position("trường", &cluster);
    assert_eq!(pos, Some(3)); // Position of 'ơ'
}

#[test]
fn test_find_phonology_position_qu_pattern() {
    // "quá" - cluster is "uá", but skip 'u' due to 'qu'
    let cluster = make_cluster(2, vec!['u', 'á'], ClusterType::Double);
    let pos = find_phonology_position("quá", &cluster);
    assert_eq!(pos, Some(3)); // Position of 'á'
}

#[test]
fn test_find_phonology_position_triple_vowel() {
    // "oai" - tone on middle vowel 'a'
    let cluster = make_cluster(0, vec!['o', 'a', 'i'], ClusterType::Triple);
    let pos = find_phonology_position("oai", &cluster);
    assert_eq!(pos, Some(1));
}

#[test]
fn test_find_phonology_position_oa_pattern() {
    // "hoa" - modern style, tone on 'a'
    let cluster = make_cluster(1, vec!['o', 'a'], ClusterType::DoubleOA);
    let pos = find_phonology_position("hoa", &cluster);
    assert_eq!(pos, Some(2)); // Modern style: tone on 'a'
}

#[test]
fn test_find_free_position_nearest() {
    // "trường" - cluster "ươ" at positions 2-3
    let cluster = make_cluster(2, vec!['ư', 'ơ'], ClusterType::CompoundUO);
    
    // Tone key typed at position 4 (after cluster)
    // Should tone the nearest vowel: 'ơ' at position 3
    let pos = find_free_position(&cluster, Some(4));
    assert_eq!(pos, Some(3));
    
    // Tone key typed at position 2 (at cluster start)
    // Should tone 'ư' at position 2
    let pos = find_free_position(&cluster, Some(2));
    assert_eq!(pos, Some(2));
}

#[test]
fn test_find_tone_position_phonology_mode() {
    let cluster = make_cluster(1, vec!['ê'], ClusterType::Single);
    let pos = find_tone_position("tên", &cluster, TonePositioningMode::Phonology, None);
    assert_eq!(pos, Some(1));
}

#[test]
fn test_find_tone_position_free_mode() {
    let cluster = make_cluster(2, vec!['ư', 'ơ'], ClusterType::CompoundUO);
    let pos = find_tone_position("trường", &cluster, TonePositioningMode::Free, Some(4));
    assert_eq!(pos, Some(3)); // Nearest to position 4 is 'ơ' at 3
}
