use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub(crate) struct CoinIconProps {
    #[prop_or_default]
    pub class: Classes,
}

/// Компонент иконки монеты
#[function_component(CoinIcon)]
pub(crate) fn coin_icon(props: &CoinIconProps) -> Html {
    html! {
        <img
            src="images/coin-icon.svg"
            alt="Coin"
            class={props.class.clone()}
        />
    }
}
