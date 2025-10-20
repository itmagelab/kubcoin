//! Internationalization (i18n) module for KubCoin landing page.
//!
//! This module provides type-safe language switching and content loading
//! with support for Russian and English languages.

pub(crate) use crate::i18n_provider::{LanguageProvider, use_language};
pub(crate) use crate::i18n_types::Language;

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
    pub(crate) aria_main_actions: String,
    pub(crate) img_alt_screenshot: String,
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
    pub(crate) features_section_subtitle: String,
    pub(crate) chats_section_title: String,
    pub(crate) security_section_title: String,
    pub(crate) security_section_subtitle: String,
    pub(crate) faq_section_title: String,
    pub(crate) pricing_section_title: String,
    pub(crate) pricing_section_subtitle: String,
    pub(crate) pricing_free_title: String,
    pub(crate) pricing_free_price: String,
    pub(crate) pricing_free_period: String,
    pub(crate) pricing_premium_title: String,
    pub(crate) pricing_premium_price: String,
    pub(crate) pricing_premium_period: String,
    pub(crate) pricing_premium_coming_soon: String,
    pub(crate) pricing_premium_in_development: String,
    pub(crate) pricing_premium_everything_plus: String,
    pub(crate) pricing_enterprise_text: String,
    pub(crate) pricing_enterprise_contact: String,
    pub(crate) pricing_enterprise_suffix: String,
    pub(crate) stats_section_title: String,
    pub(crate) stats_total_usage: String,
    pub(crate) stats_total_usage_aria: String,
    pub(crate) stats_on_premise: String,
    pub(crate) stats_on_premise_aria: String,
    pub(crate) stats_followers: String,
    pub(crate) stats_followers_aria: String,
    pub(crate) stats_likes: String,
    pub(crate) stats_likes_aria: String,
    pub(crate) feature_unlimited_operations: String,
    pub(crate) feature_all_basic_functions: String,
    pub(crate) feature_basic_analytics: String,
    pub(crate) feature_expense_categories: String,
    pub(crate) feature_math_expressions: String,
    pub(crate) feature_ai_text_recognition: String,
    pub(crate) feature_monthly_reports: String,
    pub(crate) feature_community_support: String,
    pub(crate) feature_advanced_analytics: String,
    pub(crate) feature_charts_visualization: String,
    pub(crate) feature_budget_planning: String,
    pub(crate) feature_multiple_currencies: String,
    pub(crate) feature_goals_savings: String,
    pub(crate) feature_priority_support: String,
    pub(crate) feature_extended_backups: String,
    pub(crate) notification_tip: String,
    pub(crate) notification_tip_text: String,
    pub(crate) aria_user_message: String,
    pub(crate) aria_bot_response: String,
}

/// Load translations for a specific language
pub(crate) fn load_translations(lang: Language) -> Result<I18nContent, String> {
    let content = match lang {
        Language::Russian => include_str!("../static/i18n/ru.yaml"),
        Language::English => include_str!("../static/i18n/en.yaml"),
    };

    serde_yaml::from_str(content)
        .map_err(|e| format!("Failed to parse {} translations: {}", lang, e))
}
