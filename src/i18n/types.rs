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

    /// Detect language from browser
    pub(crate) fn from_browser() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Some(navigator_lang) = window.navigator().language() {
                    let lang_code = navigator_lang.to_lowercase();
                    let lang = if lang_code.starts_with("en") {
                        Some(Language::English)
                    } else if lang_code.starts_with("ru") {
                        Some(Language::Russian)
                    } else {
                        None
                    };

                    if let Some(detected_lang) = lang {
                        tracing::info!("Detected browser language: {}", detected_lang);
                        return detected_lang;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_display() {
        assert_eq!(format!("{}", Language::Russian), "ru");
        assert_eq!(format!("{}", Language::English), "en");
    }

    #[test]
    fn test_default_language() {
        assert_eq!(Language::default(), Language::Russian);
    }
}
