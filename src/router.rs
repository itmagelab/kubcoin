//! Routing module for localized paths.
//!
//! This module defines routes for different language versions of the site:
//! - `/ru/` for Russian
//! - `/en/` for English
//! - `/` redirects to browser-detected language

use crate::i18n::Language;
use yew_router::prelude::*;

/// Application routes with language-specific paths
#[derive(Clone, Routable, PartialEq)]
pub(crate) enum Route {
    #[at("/ru")]
    Russian,
    #[at("/en")]
    English,
    #[at("/")]
    #[not_found]
    Home,
}

impl Route {
    /// Convert route to corresponding language
    pub(crate) fn to_language(&self) -> Language {
        match self {
            Route::Russian => Language::Russian,
            Route::English => Language::English,
            Route::Home => Language::from_browser(),
        }
    }

    /// Create route from language
    pub(crate) fn from_language(lang: Language) -> Self {
        match lang {
            Language::Russian => Route::Russian,
            Language::English => Route::English,
        }
    }
}
