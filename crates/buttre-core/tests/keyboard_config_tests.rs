use buttre_core::keyboard::Config;

#[test]
fn test_parse_config() {
    let toml = r#"
[metadata]
id = "telex"
name = "Telex"
language = "vietnamese"

[transformations]
"aa" = "â"
"dd" = "đ"

[tones]
"s" = "acute"
"f" = "grave"

[rules]
tone_position = "modern"
validate_syllables = true
"#;

    let config = Config::from_toml_str(toml).unwrap();
    assert_eq!(config.metadata.id, "telex");
    assert_eq!(config.transformations.get("aa"), Some(&"â".to_string()));
    assert_eq!(config.tones.get("s"), Some(&"acute".to_string()));
}
