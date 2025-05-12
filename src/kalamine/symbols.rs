use serde::Deserializer;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Symbol {
    Character(char),
    DeadKey(char),
}

impl Symbol {
    pub fn filter_empty(symbol: Option<Symbol>) -> Option<Symbol> {
        // TODO: get rid of this hack with a proper validation step
        symbol.filter(|s| *s != Symbol::Character('\x00'))
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Character(c) => write!(f, "{}", c),
            Symbol::DeadKey(c) => write!(f, "*{}", c),
        }
    }
}

impl<'de> serde::Deserialize<'de> for Symbol {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        let mut chars = s.chars();

        match (chars.next(), chars.next(), chars.next()) {
            (None, _, _) => Ok(Symbol::Character('\x00')), // Sentinel value for empty string
            (Some(first), None, _) => Ok(Symbol::Character(first)),
            (Some('*'), Some(second), None) => Ok(Symbol::DeadKey(second)),
            _ => Err(serde::de::Error::custom(format!("Invalid symbol: {s}"))),
        }
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct DeadKey {
    pub name: char,
}

impl std::fmt::Display for DeadKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "*{}", self.name)
    }
}

impl<'de> serde::Deserialize<'de> for DeadKey {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        let mut chars = s.chars();

        match (chars.next(), chars.next(), chars.next()) {
            (Some('*'), Some(second), None) => Ok(DeadKey { name: second }),
            _ => Err(serde::de::Error::custom(format!("Invalid dead key: {s}"))),
        }
    }
}


#[derive(Debug, PartialEq)]
pub struct ModMapping {
    pub base: Option<Symbol>,
    pub shift: Option<Symbol>,
    pub altgr: Option<Symbol>,
    pub altgr_shift: Option<Symbol>,
}

impl ModMapping {
    // helper function to easily create an instance in the tests
    pub fn from<T: AsRef<str>>(vec: Vec<T>) -> Self {
        let symbols: Vec<Symbol> = vec
            .into_iter()
            .map(|s| serde_json::from_str(&format!(r#""{}""#, s.as_ref())))
            .collect::<Result<_, _>>()
            .unwrap();

        let mut iter = symbols.into_iter();
        ModMapping {
            base: Symbol::filter_empty(iter.next()),
            shift: Symbol::filter_empty(iter.next()),
            altgr: Symbol::filter_empty(iter.next()),
            altgr_shift: Symbol::filter_empty(iter.next()),
        }
    }
}

impl<'de> serde::Deserialize<'de> for ModMapping {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let vec: Vec<Symbol> = Vec::deserialize(d)?;

        if vec.len() > 4 {
            return Err(serde::de::Error::invalid_length(
                vec.len(),
                &"at most 4 symbols",
            ));
        }

        let mut iter = vec.into_iter();

        Ok(ModMapping {
            base: Symbol::filter_empty(iter.next()),
            shift: Symbol::filter_empty(iter.next()),
            altgr: Symbol::filter_empty(iter.next()),
            altgr_shift: Symbol::filter_empty(iter.next()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_symbol_single_character() {
        let json = r#""a""#;
        let symbol: Symbol = serde_json::from_str(json).unwrap();
        assert_eq!(symbol, Symbol::Character('a'));
    }

    #[test]
    fn deserialize_symbol_dead_key() {
        let json = r#""*^""#;
        let symbol: Symbol = serde_json::from_str(json).unwrap();
        assert_eq!(symbol, Symbol::DeadKey('^'));
    }

    #[test]
    fn deserialize_symbol_none() {
        let json = r#""""#;
        let symbol: Symbol = serde_json::from_str(json).unwrap();
        assert_eq!(symbol, Symbol::Character('\x00'));
    }

    #[test]
    fn deserialize_symbol_invalid_extra_characters() {
        let json = r#""*ab""#;
        let result: Result<Symbol, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_symbol_invalid_dead_key_format() {
        let json = r#""ab""#;
        let result: Result<Symbol, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_symbol_escaped_char() {
        let json = r#""\\""#;
        let symbol: Symbol = serde_json::from_str(json).unwrap();
        assert_eq!(symbol, Symbol::Character('\\'));
    }

    #[test]
    fn deserialize_symbol_unicode() {
        let json = r#""ඞ""#;
        let symbol: Symbol = serde_json::from_str(json).unwrap();
        assert_eq!(symbol, Symbol::Character('ඞ'));
    }

    #[test]
    fn deserialize_dead_key() {
        let json = r#""*^""#;
        let deadkey: DeadKey = serde_json::from_str(json).unwrap();
        assert_eq!(deadkey, DeadKey { name: '^' });
    }

    #[test]
    fn deserialize_dead_key_invalid() {
        let json = r#""a""#;
        let result: Result<DeadKey, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_dead_key_extra_characters() {
        let json = r#""*ab""#;
        let result: Result<DeadKey, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn deserialize_layers() {
        let json = r#"["**", "", "a"]"#;
        let symbols: ModMapping = serde_json::from_str(json).unwrap();
        let expected = ModMapping {
            base: Some(Symbol::DeadKey('*')),
            shift: None,
            altgr: Some(Symbol::Character('a')),
            altgr_shift: None,
        };
        assert_eq!(symbols, expected);
    }

    #[test]
    fn deserialize_layers_empty() {
        let json = r#"[]"#;
        let symbols: ModMapping = serde_json::from_str(json).unwrap();
        let expected = ModMapping {
            base: None,
            shift: None,
            altgr: None,
            altgr_shift: None,
        };
        assert_eq!(symbols, expected);
    }

    #[test]
    fn deserialize_layers_too_many() {
        let json = r#"["a", "b", "c", "d", "e"]"#;
        let result: Result<ModMapping, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
