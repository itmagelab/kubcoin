//! Internationalization (i18n) module for KubCoin landing page.
//!
//! This module provides type-safe language switching and content loading
//! with support for Russian and English languages.

pub(crate) mod content;
pub(crate) mod provider;
pub(crate) mod types;

// Re-export commonly used items
pub(crate) use content::{
    ChatTranslation, DialogTranslation, FeatureTranslation, FooterTranslations, HeaderTranslations,
    I18nContent, QATranslation, SecurityTranslation, UITranslations, load_translations,
};
pub(crate) use provider::{LanguageProvider, use_language};
pub(crate) use types::Language;
