mod button;

use yew::prelude::*;

use crate::handler::{ChatItem, get_content};

#[function_component(Header)]
pub(crate) fn header() -> Html {
    html! {
        <div class="container has-text-centered">
          <div class="columns is-vcentered is-multiline">
            <div class="column is-full-mobile is-half-tablet">
              <div class="box">
                <h1 class="title is-1 has-text-weight-bold">
                            { "KubCoin" }
                </h1>
              <h2 class="subtitle is-3 has-text-grey">{ "Телеграм Бот для управления доходами и расходами" }</h2>
              </div>
              <div class="buttons">
                  <button::Start />
                  <button::Group />
                  <button::Channel />
              </div>
            </div>

            <div class="column is-full-mobile is-half-tablet">
              <figure class="image phone-fade" style="max-width: 300px; margin: 0 auto;">
                <img src="images/IMG_3089.JPG" alt="Скриншот KubCoin" />
              </figure>
            </div>
          </div>
        </div>
    }
}

#[function_component(Body)]
pub(crate) fn body() -> Html {
    html! {
        <div class="container">
            <Chats />
            <Split />
            <QA />
            <Split />
            <Usage />
        </div>
    }
}

#[function_component(Split)]
pub(crate) fn split() -> Html {
    html! {
       <section class="section">
       </section>
    }
}

#[function_component(Usage)]
pub(crate) fn usage() -> Html {
    html! {
        <div class="container">
            <nav class="level">
              <div class="level-item has-text-centered">
                <div>
                  <p class="heading">{ "Total usage" }</p>
                  <p class="title">{ 8 }</p>
                </div>
              </div>
              <div class="level-item has-text-centered">
                <div>
                  <p class="heading">{ "On-premise" }</p>
                  <p class="title">{ 1 }</p>
                </div>
              </div>
              <div class="level-item has-text-centered">
                <div>
                  <p class="heading">{ "Followers" }</p>
                  <p class="title">{ "1K" }</p>
                </div>
              </div>
              <div class="level-item has-text-centered">
                <div>
                  <p class="heading">{ "Likes" }</p>
                  <p class="title">{ 789 }</p>
                </div>
              </div>
            </nav>
        </div>
    }
}

#[function_component(QA)]
fn qa() -> Html {
    let items = get_content().qa;

    let open_index = use_state(|| None);

    html! {
        <div class="container">
            { for items.iter().enumerate().map(|(idx, item)| {
                let is_open = *open_index == Some(idx);
                let on_click = {
                    let open_index = open_index.clone();
                    Callback::from(move |_| {
                        if is_open {
                            open_index.set(None);
                        } else {
                            open_index.set(Some(idx));
                        }
                    })
                };
                html! {
                    <div class={classes!("box", if is_open { "is-active" } else { "" })}>
                        <div class="content is-medium" onclick={on_click} style="cursor: pointer;">
                            <strong>{ &item.question }</strong>
                        </div>
                        <div class="content" style={if is_open { "display: block;" } else { "display: none;" }}>
                            { &item.answer }
                        </div>
                    </div>
                }
            }) }
        </div>
    }
}

#[function_component(Chats)]
pub(crate) fn chats() -> Html {
    let content = get_content();
    html! {
        <>
            { for content.chats.chunks(2).map(|pair| html! {
                <div class="columns is-multiline">
                    <div class="column">
                    </div>
                    { for pair.iter().map(|item| html! {
                        <div class="column is-one-third">
                            { chat(item) }
                        </div>
                    }) }
                    <div class="column">
                    </div>
                </div>
            }) }
        </>
    }
}

pub(crate) fn chat(chat: &ChatItem) -> Html {
    html! {
        <div class="container" style="border: 1px solid #ccc; padding: 1rem; border-radius: 8px;">
            <div class="title"><p>{ &chat.title }</p></div>
            <div class="subtitle"><p>{ &chat.subtitle }</p></div>

            { for chat.dialogs.iter().map(|dialog| {
            html! {
            <>
                <div class="block is-flex is-justify-content-flex-end">
                    <div class="box" style="max-width: 60%;">
                        <p>{ &dialog.req }</p>
                    </div>
                </div>
                <div class="block is-flex is-justify-content-flex-start">
                    <pre class="box" style="margin: 0; white-space: pre-wrap; word-wrap: break-word; overflow-wrap: anywhere;">{ &dialog.res }</pre>
                </div>
            </>
            }

            }) }
        </div>
    }
}

#[function_component(Footer)]
pub(crate) fn footer() -> Html {
    html! {
        <footer class="footer">
            <div class="content has-text-centered">
                <div class="content has-text-centered">
                    <p>{ "© iTmageLAB. All rights reserved." }</p>
                </div>
            </div>
        </footer>
    }
}
