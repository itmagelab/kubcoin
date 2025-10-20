//! Navbar component for site navigation with language switcher.

use super::LanguageSwitcher;
use yew::prelude::*;

/// Top navigation bar with logo and language switcher
#[function_component(Navbar)]
pub(crate) fn navbar() -> Html {
    let is_active = use_state(|| false);

    let toggle_burger = {
        let is_active = is_active.clone();
        Callback::from(move |_| {
            is_active.set(!*is_active);
        })
    };

    let navbar_menu_class = if *is_active {
        "navbar-menu is-active"
    } else {
        "navbar-menu"
    };

    let burger_class = if *is_active {
        "navbar-burger is-active"
    } else {
        "navbar-burger"
    };

    html! {
        <nav class="navbar is-dark is-fixed-top" role="navigation" aria-label="main navigation" style="box-shadow: 0 2px 4px rgba(0,0,0,0.3); z-index: 30;">
            <div class="container">
                <div class="navbar-brand">
                    <a class="navbar-item" href="/" style="font-weight: bold; font-size: 1.25rem;">
                        <span class="icon" style="font-size: 1.5rem;">
                            <i class="fas fa-coins"></i>
                        </span>
                        <span style="margin-left: 0.5rem;">{ "KubCoin" }</span>
                    </a>

                    <a
                        role="button"
                        class={burger_class}
                        aria-label="menu"
                        aria-expanded={is_active.to_string()}
                        onclick={toggle_burger}
                    >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </a>
                </div>

                <div class={navbar_menu_class}>
                    <div class="navbar-end">
                        <div class="navbar-item">
                            <LanguageSwitcher />
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    }
}
