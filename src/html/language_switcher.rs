//! Language switcher component for changing the UI language.

use crate::i18n::{Language, use_language};
use yew::prelude::*;

/// Language switcher component with dropdown menu
#[function_component(LanguageSwitcher)]
pub(crate) fn language_switcher() -> Html {
    let ctx = use_language();
    let current_lang = ctx.language;
    let is_open = use_state(|| false);

    let toggle_dropdown = {
        let is_open = is_open.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            is_open.set(!*is_open);
        })
    };

    let close_dropdown = {
        let is_open = is_open.clone();
        Callback::from(move |_| {
            is_open.set(false);
        })
    };

    let on_language_select = {
        let set_language = ctx.set_language.clone();
        let is_open = is_open.clone();

        Callback::from(move |lang: Language| {
            set_language.emit(lang);
            is_open.set(false);
        })
    };

    let dropdown_class = if *is_open {
        "dropdown is-active"
    } else {
        "dropdown"
    };

    html! {
        <div class={dropdown_class}>
            <div class="dropdown-trigger">
                <button
                    class="button is-light"
                    aria-haspopup="true"
                    aria-controls="language-menu"
                    aria-label="Select language"
                    onclick={toggle_dropdown}
                >
                    <span class="icon is-small">
                        <span>{ current_lang.flag() }</span>
                    </span>
                    <span>{ current_lang.native_name() }</span>
                    <span class="icon is-small">
                        <i class="fas fa-angle-down" aria-hidden="true"></i>
                    </span>
                </button>
            </div>
            <div class="dropdown-menu" id="language-menu" role="menu">
                <div class="dropdown-content">
                    { for Language::all().iter().map(|&lang| {
                        let on_click = {
                            let on_language_select = on_language_select.clone();
                            Callback::from(move |e: MouseEvent| {
                                e.prevent_default();
                                on_language_select.emit(lang);
                            })
                        };

                        let item_class = if lang == current_lang {
                            "dropdown-item is-active"
                        } else {
                            "dropdown-item"
                        };

                        html! {
                            <a
                                href="#"
                                class={item_class}
                                onclick={on_click}
                                role="menuitem"
                            >
                                <span class="icon-text">
                                    <span class="icon">
                                        <span>{ lang.flag() }</span>
                                    </span>
                                    <span>{ lang.native_name() }</span>
                                </span>
                            </a>
                        }
                    }) }
                </div>
            </div>

            // Close dropdown when clicking outside
            if *is_open {
                <div
                    class="modal-background"
                    style="position: fixed; top: 0; left: 0; width: 100vw; height: 100vh; background: transparent; z-index: 10;"
                    onclick={close_dropdown}
                />
            }
        </div>
    }
}
