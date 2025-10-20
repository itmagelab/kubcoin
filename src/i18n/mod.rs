//! Internationalization (i18n) module for KubCoin landing page.
//!
//! This module provides type-safe language switching and content loading
//! with support for Russian and English languages.

mod provider;
mod types;

pub(crate) use provider::{LanguageProvider, use_language};
pub(crate) use types::Language;

use serde::Deserialize;

/// Main content structure that holds all translations
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub(crate) struct I18nContent {
    pub(crate) header: HeaderTranslations,
    pub(crate) features: Vec<FeatureTranslation>,
    pub(crate) chats: Vec<ChatTranslation>,
    pub(crate) security: Vec<SecurityTranslation>,
    pub(crate) qa: Vec<QATranslation>,
    pub(crate) footer: FooterTranslations,
    pub(crate) ui: UITranslations,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub(crate) struct HeaderTranslations {
    pub(crate) title: String,
    pub(crate) subtitle: String,
    pub(crate) cta_button: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub(crate) struct FeatureTranslation {
    pub(crate) icon: String,
    pub(crate) title: String,
    pub(crate) description: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub(crate) struct ChatTranslation {
    pub(crate) title: String,
    pub(crate) subtitle: String,
    pub(crate) dialogs: Vec<DialogTranslation>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub(crate) struct DialogTranslation {
    pub(crate) req: String,
    pub(crate) res: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub(crate) struct SecurityTranslation {
    pub(crate) icon: String,
    pub(crate) title: String,
    pub(crate) description: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub(crate) struct QATranslation {
    pub(crate) question: String,
    pub(crate) answer: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub(crate) struct FooterTranslations {
    pub(crate) copyright: String,
    pub(crate) developed_by: String,
    pub(crate) privacy_policy: String,
    pub(crate) terms_of_service: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub(crate) struct UITranslations {
    pub(crate) start_bot: String,
    pub(crate) join_group: String,
    pub(crate) join_channel: String,
    pub(crate) learn_more: String,
    pub(crate) features_section_title: String,
    pub(crate) chats_section_title: String,
    pub(crate) security_section_title: String,
    pub(crate) faq_section_title: String,
}

/// Load translations for a specific language
pub(crate) fn load_translations(lang: Language) -> Result<I18nContent, String> {
    let content = match lang {
        Language::Russian => include_str!("../../static/i18n/ru.yaml"),
        Language::English => include_str!("../../static/i18n/en.yaml"),
    };

    serde_yaml::from_str(content)
        .map_err(|e| format!("Failed to parse {} translations: {}", lang, e))
}
