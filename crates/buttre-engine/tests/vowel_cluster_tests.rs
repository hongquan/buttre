use buttre_engine::vowel::cluster::{find_vowel_clusters, classify_cluster, is_vowel, normalize_vowel, VowelCluster, ClusterType};

#[test]
fn test_is_vowel() {
    // Basic vowels
    assert!(is_vowel('a'));
    assert!(is_vowel('e'));
    assert!(is_vowel('i'));
    assert!(is_vowel('o'));
    assert!(is_vowel('u'));
    assert!(is_vowel('y'));
    
    // Marked vowels
    assert!(is_vowel('ă'));
    assert!(is_vowel('â'));
    assert!(is_vowel('ê'));
    assert!(is_vowel('ô'));
    assert!(is_vowel('ơ'));
    assert!(is_vowel('ư'));
    
    // Toned vowels
    assert!(is_vowel('á'));
    assert!(is_vowel('à'));
    assert!(is_vowel('ế'));
    assert!(is_vowel('ề'));
    
    // Non-vowels
    assert!(!is_vowel('b'));
    assert!(!is_vowel('c'));
    assert!(!is_vowel('đ'));
    assert!(!is_vowel('1'));
    assert!(!is_vowel(' '));
}

#[test]
fn test_find_vowel_clusters_single() {
    let clusters = find_vowel_clusters("bat");
    assert_eq!(clusters.len(), 1);
    assert_eq!(clusters[0].vowels, vec!['a']);
    assert_eq!(clusters[0].cluster_type, ClusterType::Single);
}

#[test]
fn test_find_vowel_clusters_double() {
    let clusters = find_vowel_clusters("hai");
    assert_eq!(clusters.len(), 1);
    assert_eq!(clusters[0].vowels, vec!['a', 'i']);
    assert_eq!(clusters[0].cluster_type, ClusterType::Double);
}

#[test]
fn test_find_vowel_clusters_compound_uo() {
    let clusters = find_vowel_clusters("trường");
    assert_eq!(clusters.len(), 1);
    assert_eq!(clusters[0].vowels, vec!['ư', 'ơ']);
    assert_eq!(clusters[0].cluster_type, ClusterType::CompoundUO);
}

#[test]
fn test_find_vowel_clusters_triple() {
    let clusters = find_vowel_clusters("cười");
    assert_eq!(clusters.len(), 1);
    assert_eq!(clusters[0].vowels, vec!['ư', 'ơ', 'i']);
    assert_eq!(clusters[0].cluster_type, ClusterType::Triple);
}

#[test]
fn test_find_vowel_clusters_multiple() {
    let clusters = find_vowel_clusters("hoàn");
    // Should find "oà" cluster (normalized to "oa")
    assert_eq!(clusters.len(), 1);
    assert_eq!(clusters[0].vowels, vec!['o', 'a']);  // Normalized
}

#[test]
fn test_find_vowel_clusters_oa_pattern() {
    let clusters = find_vowel_clusters("hoa");
    assert_eq!(clusters.len(), 1);
    assert_eq!(clusters[0].vowels, vec!['o', 'a']);
    assert_eq!(clusters[0].cluster_type, ClusterType::DoubleOA);
}

#[test]
fn test_classify_cluster_single() {
    assert_eq!(classify_cluster(&['a']), ClusterType::Single);
    assert_eq!(classify_cluster(&['ư']), ClusterType::Single);
}

#[test]
fn test_classify_cluster_compound_uo() {
    assert_eq!(classify_cluster(&['u', 'o']), ClusterType::CompoundUO);
    assert_eq!(classify_cluster(&['u', 'ơ']), ClusterType::CompoundUO);
    assert_eq!(classify_cluster(&['ư', 'o']), ClusterType::CompoundUO);
    assert_eq!(classify_cluster(&['ư', 'ơ']), ClusterType::CompoundUO);
}

#[test]
fn test_classify_cluster_double_oa() {
    assert_eq!(classify_cluster(&['o', 'a']), ClusterType::DoubleOA);
    assert_eq!(classify_cluster(&['o', 'e']), ClusterType::DoubleOA);
}

#[test]
fn test_classify_cluster_triple() {
    assert_eq!(classify_cluster(&['u', 'ơ', 'i']), ClusterType::Triple);
    assert_eq!(classify_cluster(&['o', 'a', 'i']), ClusterType::Triple);
}

#[test]
fn test_vowel_cluster_contains_position() {
    let cluster = VowelCluster {
        start_pos: 2,
        end_pos: 4,
        vowels: vec!['ư', 'ơ'],
        cluster_type: ClusterType::CompoundUO,
    };
    
    assert!(!cluster.contains_position(1));
    assert!(cluster.contains_position(2));
    assert!(cluster.contains_position(3));
    assert!(!cluster.contains_position(4));
}

#[test]
fn test_vowel_cluster_to_string() {
    let cluster = VowelCluster {
        start_pos: 0,
        end_pos: 2,
        vowels: vec!['ư', 'ơ'],
        cluster_type: ClusterType::CompoundUO,
    };
    
    assert_eq!(cluster.to_string(), "ươ");
}
