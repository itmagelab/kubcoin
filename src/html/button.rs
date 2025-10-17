use yew::prelude::*;

#[function_component(Start)]
pub(crate) fn start() -> Html {
    link(
        "Начать",
        "https://t.me/kubcoin_bot",
        "Открыть KubCoin бот в Telegram",
        false,
    )
}

#[function_component(Group)]
pub(crate) fn group() -> Html {
    link(
        "Группа в Telegram",
        "https://t.me/itmagelab_ru_group",
        "Присоединиться к группе iTmageLab в Telegram",
        true,
    )
}

#[function_component(Channel)]
pub(crate) fn channel() -> Html {
    link(
        "Подписаться на канал",
        "https://t.me/itmagelab_ru",
        "Подписаться на новостной канал iTmageLab в Telegram",
        true,
    )
}

pub(crate) fn link(
    text: &'static str,
    url: &'static str,
    aria_label: &'static str,
    primary: bool,
) -> Html {
    let class = if primary {
        "button is-primary is-medium is-outlined"
    } else {
        "button is-link is-medium"
    };
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
