use yew::prelude::*;

#[function_component(Start)]
pub(crate) fn start() -> Html {
    link("Начать", "https://t.me/kubcoin_bot", false)
}

#[function_component(Group)]
pub(crate) fn group() -> Html {
    link("Группа в Telegram", "https://t.me/itmagelab_ru_group", true)
}

#[function_component(Channel)]
pub(crate) fn channel() -> Html {
    link("Подписаться на канал", "https://t.me/itmagelab_ru", true)
}

pub(crate) fn link(text: &'static str, url: &'static str, primary: bool) -> Html {
    let class = if primary {
        "button is-primary is-medium is-outlined"
    } else {
        "button is-link is-medium"
    };
    html! {
         <a href={ url }
            target="_blank"
            rel="noopener noreferrer"
            class={ class }>
            { text }
         </a>

    }
}
