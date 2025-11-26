//! Language provider and context for managing i18n state.

use super::content::{I18nContent, load_translations};
use super::types::Language;
use crate::router::Route;
use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::*;

#[allow(dead_code)]
const STORAGE_KEY: &str = "kubcoin_language";

/// Context that holds the current language and translations
#[derive(Clone, PartialEq)]
pub(crate) struct LanguageContext {
    pub(crate) language: Language,
    pub(crate) translations: Rc<I18nContent>,
    pub(crate) set_language: Callback<Language>,
}

/// Props for LanguageProvider component
#[derive(Properties, PartialEq)]
pub(crate) struct LanguageProviderProps {
    #[prop_or_default]
    pub(crate) children: Children,
    /// Initial language from router (takes precedence over storage/browser detection)
    #[prop_or_default]
    pub(crate) initial_language: Option<Language>,
}

/// Provider component that manages language state
#[function_component(LanguageProvider)]
pub(crate) fn language_provider(props: &LanguageProviderProps) -> Html {
    let navigator = use_navigator().expect("LanguageProvider must be used within HashRouter");

    // Prioritize: 1) Router-provided language, 2) LocalStorage, 3) Browser detection
    let initial_lang = props
        .initial_language
        .unwrap_or_else(|| load_from_storage().unwrap_or_else(Language::from_browser));

    let language = use_state(|| initial_lang);
    let translations = use_state(|| {
        let lang = *language;
        Rc::new(load_translations(lang).unwrap_or_else(|e| {
            tracing::error!("Failed to load translations: {}", e);
            // Fallback to Russian
            load_translations(Language::Russian).expect("Failed to load Russian translations")
        }))
    });

    // Sync state with props.initial_language when it changes (URL change)
    {
        let language = language.clone();
        let translations = translations.clone();
        use_effect_with(props.initial_language, move |&new_route_lang| {
            if let Some(new_lang) = new_route_lang {
                if *language != new_lang {
                    tracing::info!("Route changed, updating language to: {}", new_lang);
                    match load_translations(new_lang) {
                        Ok(content) => {
                            language.set(new_lang);
                            translations.set(Rc::new(content));
                            save_to_storage(new_lang);
                            update_html_lang(new_lang);
                        }
                        Err(e) => {
                            tracing::error!("Failed to load translations for {}: {}", new_lang, e);
                        }
                    }
                }
            }
            || ()
        });
    }

    let set_language = {
        Callback::from(move |new_lang: Language| {
            // Instead of setting state directly, we push a new route.
            // The route change will trigger the effect above, which updates the state.
            let route = Route::from_language(new_lang);
            navigator.push(&route);
        })
    };

    // Update HTML lang attribute on mount
    use_effect_with(*language, |lang| {
        update_html_lang(*lang);
        || ()
    });

    let context = LanguageContext {
        language: *language,
        translations: (*translations).clone(),
        set_language,
    };

    html! {
        <ContextProvider<LanguageContext> context={context}>
            { for props.children.iter() }
        </ContextProvider<LanguageContext>>
    }
}

/// Hook to access language context
#[hook]
pub(crate) fn use_language() -> LanguageContext {
    use_context::<LanguageContext>().expect("use_language must be used within LanguageProvider")
}

/// Save language preference to LocalStorage
#[allow(unused_variables)]
fn save_to_storage(lang: Language) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let lang_code = lang.code();
                if let Err(e) = storage.set_item(STORAGE_KEY, lang_code) {
                    tracing::warn!("Failed to save language to storage: {:?}", e);
                } else {
                    tracing::debug!("Language saved to storage: {}", lang_code);
                }
            }
        }
    }
}

/// Load language preference from LocalStorage
fn load_from_storage() -> Option<Language> {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(lang_code)) = storage.get_item(STORAGE_KEY) {
                    let lang = match lang_code.as_str() {
                        "ru" => Some(Language::Russian),
                        "en" => Some(Language::English),
                        _ => None,
                    };
                    if let Some(detected_lang) = lang {
                        tracing::info!("Loaded language from storage: {}", detected_lang);
                        return Some(detected_lang);
                    }
                }
            }
        }
    }
    None
}

/// Update HTML lang attribute
#[allow(unused_variables)]
fn update_html_lang(lang: Language) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(html_element) = document.document_element() {
                    let lang_code = lang.code();
                    if let Err(e) = html_element.set_attribute("lang", lang_code) {
                        tracing::warn!("Failed to update HTML lang attribute: {:?}", e);
                    } else {
                        tracing::debug!("HTML lang attribute updated to: {}", lang_code);
                    }
                }
            }
        }
    }
}
