use serde::Deserializer;


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum KeyAction {
    Symbol(char),
    DeadKey(char),
}

impl<'de> serde::Deserialize<'de> for KeyAction {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        let mut chars = s.chars();

        match (chars.next(), chars.next(), chars.next()) {
            (None, _, _) => Ok(KeyAction::Symbol('\x00')), // Signal an empty string
            (Some(first), None, _) => Ok(KeyAction::Symbol(first)),
            (Some('*'), Some(second), None) => Ok(KeyAction::DeadKey(second)),
            _ => Err(serde::de::Error::custom(format!("Invalid symbol: {s}"))),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct LayerMapping {
    pub base: Option<KeyAction>,
    pub shift: Option<KeyAction>,
    pub altgr: Option<KeyAction>,
    pub altgr_shift: Option<KeyAction>,
}

impl<'de> serde::Deserialize<'de> for LayerMapping {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let vec: Vec<KeyAction> = Vec::deserialize(d)?;

        if vec.len() > 4 {
            return Err(serde::de::Error::invalid_length(
                vec.len(),
                &"at most 4 symbols",
            ));
        }

        let handle_empty =
            |symbol: Option<KeyAction>| symbol.filter(|s| *s != KeyAction::Symbol('\x00'));
        let mut iter = vec.into_iter();

        Ok(LayerMapping {
            base: handle_empty(iter.next()),
            shift: handle_empty(iter.next()),
            altgr: handle_empty(iter.next()),
            altgr_shift: handle_empty(iter.next()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_symbol_single_character() {
        let json = r#""a""#;
        let symbol: KeyAction = serde_json::from_str(json).unwrap();
        assert_eq!(symbol, KeyAction::Symbol('a'));
    }

    #[test]
    fn deserialize_symbol_dead_key() {
        let json = r#""**""#;
        let symbol: KeyAction = serde_json::from_str(json).unwrap();
        assert_eq!(symbol, KeyAction::DeadKey('*'));
    }

    #[test]
    fn deserialize_symbol_none() {
        let json = r#""""#;
        let symbol: KeyAction = serde_json::from_str(json).unwrap();
        assert_eq!(symbol, KeyAction::Symbol('\x00'));
    }

    #[test]
    fn deserialize_symbol_invalid_extra_characters() {
        let json = r#""*ab""#;
        let result: Result<KeyAction, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_symbol_invalid_dead_key_format() {
        let json = r#""ab""#;
        let result: Result<KeyAction, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_symbol_escaped_char() {
        let json = r#""\\""#;
        let symbol: KeyAction = serde_json::from_str(json).unwrap();
        assert_eq!(symbol, KeyAction::Symbol('\\'));
    }

    #[test]
    fn deserialize_symbol_unicode() {
        let json = r#""ඞ""#;
        let symbol: KeyAction = serde_json::from_str(json).unwrap();
        assert_eq!(symbol, KeyAction::Symbol('ඞ'));
    }

    #[test]
    fn deserialize_layers() {
        let json = r#"["**", "", "a"]"#;
        let layers: LayerMapping = serde_json::from_str(json).unwrap();
        let expected = LayerMapping {
            base: Some(KeyAction::DeadKey('*')),
            shift: None,
            altgr: Some(KeyAction::Symbol('a')),
            altgr_shift: None,
        };
        assert_eq!(layers, expected);
    }

    #[test]
    fn deserialize_layers_empty() {
        let json = r#"[]"#;
        let layers: LayerMapping = serde_json::from_str(json).unwrap();
        let expected = LayerMapping {
            base: None,
            shift: None,
            altgr: None,
            altgr_shift: None,
        };
        assert_eq!(layers, expected);
    }

    #[test]
    fn deserialize_layers_too_many() {
        let json = r#"["a", "b", "c", "d", "e"]"#;
        let result: Result<LayerMapping, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
