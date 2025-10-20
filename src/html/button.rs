use crate::i18n::use_language;
use yew::prelude::*;

#[function_component(Start)]
pub(crate) fn start() -> Html {
    let ctx = use_language();
    let text = ctx.translations.ui.start_bot.clone();

    link(&text, "https://t.me/kubcoin_bot", &text, false)
}

#[function_component(Group)]
pub(crate) fn group() -> Html {
    let ctx = use_language();
    let text = ctx.translations.ui.join_group.clone();

    link(&text, "https://t.me/itmagelab_ru_group", &text, true)
}

#[function_component(Channel)]
pub(crate) fn channel() -> Html {
    let ctx = use_language();
    let text = ctx.translations.ui.join_channel.clone();

    link(&text, "https://t.me/itmagelab_ru", &text, true)
}

pub(crate) fn link(text: &str, url: &'static str, aria_label: &str, primary: bool) -> Html {
    let class = if primary {
        "button is-primary is-medium is-outlined"
    } else {
        "button is-link is-medium"
    };
    let text = text.to_string();
    let aria_label = aria_label.to_string();

    html! {
         <a href={ url }
            target="_blank"
            rel="noopener noreferrer"
            class={ class }
            aria-label={ aria_label }>
            { text }
         </a>

    }
}
