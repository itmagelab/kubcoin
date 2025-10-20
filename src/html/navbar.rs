//! Navbar component for site navigation with language switcher.

use super::LanguageSwitcher;
use yew::prelude::*;

/// Top navigation bar with language switcher
#[function_component(Navbar)]
pub(crate) fn navbar() -> Html {
    html! {
        <nav class="navbar is-fixed-top" role="navigation" aria-label="main navigation" style="background-color: #f5f5f5; box-shadow: 0 2px 4px rgba(0,0,0,0.1); z-index: 30;">
            <div class="container">
                <div class="navbar-menu" style="display: flex; justify-content: flex-end;">
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
