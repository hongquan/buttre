use buttre_core::keyboard::{Config, KeyboardBuilder};

#[test]
fn test_builder() {
    let toml = r#"
[metadata]
id = "telex"
name = "Telex"
language = "vietnamese"

[transformations]
"aa" = "â"

[tones]
"s" = "acute"

[rules]
tone_position = "modern"
"#;

    let config = Config::from_toml_str(toml).unwrap();
    let keyboard = KeyboardBuilder::new()
        .with_config(config)
        .with_language("vietnamese")
        .build();

    assert!(keyboard.is_ok());
}
