//! Language provider and context for managing i18n state.

use super::content::{I18nContent, load_translations};
use super::types::Language;
use std::rc::Rc;
use yew::prelude::*;

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

    let set_language = {
        let language = language.clone();
        let translations = translations.clone();

        Callback::from(move |new_lang: Language| {
            if *language != new_lang {
                tracing::info!("Switching language to: {}", new_lang);

                // Load new translations
                match load_translations(new_lang) {
                    Ok(content) => {
                        language.set(new_lang);
                        translations.set(Rc::new(content));
                        save_to_storage(new_lang);
                        update_html_lang(new_lang);
                        update_route(new_lang);
                        tracing::info!("Language switched successfully to: {}", new_lang);
                    }
                    Err(e) => {
                        tracing::error!("Failed to switch language: {}", e);
                    }
                }
            }
        })
    };

    // Update HTML lang attribute on mount and when language changes
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

/// Update route when language is changed
#[allow(unused_variables)]
fn update_route(lang: Language) {
    #[cfg(target_arch = "wasm32")]
    {
        use crate::router::Route;

        // Get the current location and update it
        if let Some(window) = web_sys::window() {
            let target_route = Route::from_language(lang);
            let path = match target_route {
                Route::Russian => "/ru",
                Route::English => "/en",
                Route::Home => "/",
            };

            if let Some(history) = window.history().ok() {
                if let Err(e) =
                    history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(path))
                {
                    tracing::warn!("Failed to update route: {:?}", e);
                } else {
                    tracing::debug!("Route updated to: {}", path);
                }
            }
        }
    }
}
