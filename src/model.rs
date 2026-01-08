use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Model {
    Sonnet,
    Opus,
    Haiku,
    Custom(String),
}

impl Model {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Sonnet => "claude-sonnet-4-5-20250929",
            Self::Opus => "claude-opus-4-5-20250929",
            Self::Haiku => "claude-haiku-4-5-20251001",
            Self::Custom(s) => s,
        }
    }
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<&str> for Model {
    fn from(s: &str) -> Self {
        match s {
            "sonnet" | "sonnet-4-5" | "claude-sonnet-4-5-20250929" => Self::Sonnet,
            "opus" | "opus-4-5" | "claude-opus-4-5-20250929" => Self::Opus,
            "haiku" | "haiku-4-5" | "claude-haiku-4-5-20251001" => Self::Haiku,
            _ => Self::Custom(s.to_owned()),
        }
    }
}

impl From<String> for Model {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}
