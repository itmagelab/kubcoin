use crate::i18n::use_language;
use yew::prelude::*;

#[function_component(Start)]
pub(crate) fn start() -> Html {
    let ctx = use_language();
    let text = ctx.translations.ui.start_bot.clone();

    link_medium(&text, "https://t.me/kubcoin_bot", &text, true)
}

#[function_component(Group)]
pub(crate) fn group() -> Html {
    let ctx = use_language();
    let text = ctx.translations.ui.join_group.clone();

    link(&text, "https://t.me/itmagelab_ru_group", &text, false)
}

#[function_component(Channel)]
pub(crate) fn channel() -> Html {
    let ctx = use_language();
    let text = ctx.translations.ui.join_channel.clone();

    link(&text, "https://t.me/itmagelab_ru", &text, false)
}

pub(crate) fn link(text: &str, url: &'static str, aria_label: &str, primary: bool) -> Html {
    let base_classes = "inline-flex items-center justify-center px-4 py-2 rounded-lg font-semibold transition-all hover-lift focus:outline-none focus:ring-2 focus:ring-offset-2";

    let (color_classes, focus_classes) = if primary {
        (
            "bg-blue-600 text-white hover:bg-blue-700",
            "focus:ring-blue-500",
        )
    } else {
        (
            "bg-gray-200 text-gray-800 hover:bg-gray-300 border border-gray-300",
            "focus:ring-gray-500",
        )
    };

    let text = text.to_string();
    let aria_label = aria_label.to_string();

    html! {
         <a
            href={ url }
            target="_blank"
            rel="noopener noreferrer"
            class={format!("{} {} {}", base_classes, color_classes, focus_classes)}
            aria-label={ aria_label }>
            { text }
         </a>
    }
}

pub(crate) fn link_medium(text: &str, url: &'static str, aria_label: &str, primary: bool) -> Html {
    let base_classes = "inline-flex items-center justify-center px-6 py-3 rounded-lg font-semibold text-lg transition-all hover-lift focus:outline-none focus:ring-2 focus:ring-offset-2";

    let (color_classes, focus_classes) = if primary {
        (
            "bg-blue-600 text-white hover:bg-blue-700",
            "focus:ring-blue-500",
        )
    } else {
        (
            "bg-gray-200 text-gray-800 hover:bg-gray-300 border border-gray-300",
            "focus:ring-gray-500",
        )
    };

    let text = text.to_string();
    let aria_label = aria_label.to_string();

    html! {
         <a
            href={ url }
            target="_blank"
            rel="noopener noreferrer"
            class={format!("{} {} {}", base_classes, color_classes, focus_classes)}
            aria-label={ aria_label }>
            { text }
         </a>
    }
}
