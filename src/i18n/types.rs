//! Type definitions for i18n system.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Supported languages for the KubCoin landing page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) enum Language {
    #[serde(rename = "ru")]
    Russian,
    #[serde(rename = "en")]
    English,
}

impl Language {
    /// Get language code (ISO 639-1)
    pub(crate) fn code(&self) -> &'static str {
        match self {
            Language::Russian => "ru",
            Language::English => "en",
        }
    }

    /// Get language name in its native form
    pub(crate) fn native_name(&self) -> &'static str {
        match self {
            Language::Russian => "Русский",
            Language::English => "English",
        }
    }

    /// Get language flag emoji
    pub(crate) fn flag(&self) -> &'static str {
        match self {
            Language::Russian => "🇷🇺",
            Language::English => "🇬🇧",
        }
    }

    /// Parse language from string code
    pub(crate) fn from_code(code: &str) -> Option<Self> {
        match code.to_lowercase().as_str() {
            "ru" | "rus" | "russian" => Some(Language::Russian),
            "en" | "eng" | "english" => Some(Language::English),
            _ => None,
        }
    }

    /// Get all supported languages
    pub(crate) fn all() -> &'static [Language] {
        &[Language::Russian, Language::English]
    }

    /// Detect language from browser
    pub(crate) fn from_browser() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Some(navigator_lang) = window.navigator().language() {
                    if let Some(lang) = Self::from_code(&navigator_lang) {
                        tracing::info!("Detected browser language: {}", lang);
                        return lang;
                    }
                }
            }
        }

        // Default to Russian
        tracing::info!("Using default language: Russian");
        Language::Russian
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::Russian
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

/// Wrapper type for translations content
pub(crate) type Translations = super::I18nContent;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_code() {
        assert_eq!(Language::Russian.code(), "ru");
        assert_eq!(Language::English.code(), "en");
    }

    #[test]
    fn test_language_from_code() {
        assert_eq!(Language::from_code("ru"), Some(Language::Russian));
        assert_eq!(Language::from_code("en"), Some(Language::English));
        assert_eq!(Language::from_code("RU"), Some(Language::Russian));
        assert_eq!(Language::from_code("EN"), Some(Language::English));
        assert_eq!(Language::from_code("rus"), Some(Language::Russian));
        assert_eq!(Language::from_code("english"), Some(Language::English));
        assert_eq!(Language::from_code("unknown"), None);
    }

    #[test]
    fn test_language_display() {
        assert_eq!(format!("{}", Language::Russian), "ru");
        assert_eq!(format!("{}", Language::English), "en");
    }

    #[test]
    fn test_language_all() {
        let langs = Language::all();
        assert_eq!(langs.len(), 2);
        assert!(langs.contains(&Language::Russian));
        assert!(langs.contains(&Language::English));
    }

    #[test]
    fn test_default_language() {
        assert_eq!(Language::default(), Language::Russian);
    }
}
