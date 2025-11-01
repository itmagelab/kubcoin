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

    let base_classes = "inline-flex items-center justify-center px-3 py-1 rounded-md text-sm font-medium transition-all focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500";

    let ru_classes = if current_lang == Language::Russian {
        "bg-green-600 text-white hover:bg-green-700"
    } else {
        "bg-gray-200 text-gray-800 hover:bg-gray-300"
    };

    let en_classes = if current_lang == Language::English {
        "bg-green-600 text-white hover:bg-green-700"
    } else {
        "bg-gray-200 text-gray-800 hover:bg-gray-300"
    };

    html! {
        <div class="flex items-center space-x-1 bg-white rounded-lg shadow-sm p-1 border border-gray-200">
            <button
                class={format!("{} {}", base_classes, en_classes)}
                onclick={switch_to_english}
                aria-label="Switch to English"
                aria-pressed={if current_lang == Language::English { "true" } else { "false" }}
            >
                { "Eng" }
            </button>
            <button
                class={format!("{} {}", base_classes, ru_classes)}
                onclick={switch_to_russian}
                aria-label="Переключить на русский"
                aria-pressed={if current_lang == Language::Russian { "true" } else { "false" }}
            >
                { "Ru" }
            </button>
        </div>
    }
}
