//! Language switcher component for changing the UI language.

use crate::i18n::{Language, use_language};
use yew::prelude::*;

/// Language switcher component with button toggle
#[function_component(LanguageSwitcher)]
pub(crate) fn language_switcher() -> Html {
    let ctx = use_language();
    let current_lang = ctx.language;

    let switch_to_russian = {
        let set_language = ctx.set_language.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            set_language.emit(Language::Russian);
        })
    };

    let switch_to_english = {
        let set_language = ctx.set_language.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            set_language.emit(Language::English);
        })
    };

    let ru_class = if current_lang == Language::Russian {
        "button is-small is-primary"
    } else {
        "button is-small is-light"
    };

    let en_class = if current_lang == Language::English {
        "button is-small is-primary"
    } else {
        "button is-small is-light"
    };

    html! {
        <div class="buttons has-addons is-small language-switcher" style="margin-bottom: 0; font-size: 0.75rem;">
            <button
                class={en_class}
                onclick={switch_to_english}
                aria-label="Switch to English"
                aria-pressed={if current_lang == Language::English { "true" } else { "false" }}
                style="min-width: auto; padding: 0.25rem 0.5rem; font-size: 0.75rem;"
            >
                { "Eng" }
            </button>
            <button
                class={ru_class}
                onclick={switch_to_russian}
                aria-label="Переключить на русский"
                aria-pressed={if current_lang == Language::Russian { "true" } else { "false" }}
                style="min-width: auto; padding: 0.25rem 0.5rem; font-size: 0.75rem;"
            >
                { "Ru" }
            </button>
        </div>
    }
}
