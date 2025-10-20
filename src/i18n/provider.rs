//! Language provider and context for managing i18n state.

use super::{I18nContent, load_translations, types::Language};
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
    pub(crate) children: Children,
}

/// Provider component that manages language state
#[function_component(LanguageProvider)]
pub(crate) fn language_provider(props: &LanguageProviderProps) -> Html {
    let language = use_state(|| load_from_storage().unwrap_or_else(Language::from_browser));
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
                        tracing::info!("Language switched successfully to: {}", new_lang);
                    }
                    Err(e) => {
                        tracing::error!("Failed to switch language: {}", e);
                    }
                }
            }
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
                    if let Some(lang) = Language::from_code(&lang_code) {
                        tracing::info!("Loaded language from storage: {}", lang);
                        return Some(lang);
                    }
                }
            }
        }
    }
    None
}

/// Update HTML lang attribute
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
