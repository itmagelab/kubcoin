mod button;

use yew::prelude::*;

use crate::handler::{ChatItem, get_content};

#[function_component(Header)]
pub(crate) fn header() -> Html {
    html! {
        <header role="banner">
            <div class="container has-text-centered">
              <div class="columns is-vcentered is-multiline">
                <div class="column is-full-mobile is-half-tablet">
                  <div class="box">
                    <h1 class="title is-1 has-text-weight-bold">
                                { "KubCoin" }
                    </h1>
                  <h2 class="subtitle is-3 has-text-grey">{ "Телеграм Бот для управления доходами и расходами" }</h2>
                  </div>
                  <nav class="buttons" aria-label="Основные действия">
                      <button::Start />
                      <button::Group />
                      <button::Channel />
                  </nav>
                </div>

                <div class="column is-full-mobile is-half-tablet">
                  <figure class="image phone-fade" style="max-width: 300px; margin: 0 auto;">
                    <img src="images/IMG_3089.JPG" alt="Скриншот интерфейса приложения KubCoin с примером управления финансами" loading="eager" />
                  </figure>
                </div>
              </div>
            </div>
        </header>
    }
}

#[function_component(Body)]
pub(crate) fn body() -> Html {
    html! {
        <div class="container">
            <section id="examples" aria-labelledby="examples-heading">
                <Chats />
            </section>
            <Split />
            <section id="faq" aria-labelledby="faq-heading">
                <QA />
            </section>
            <Split />
            <section id="statistics" aria-labelledby="stats-heading">
                <Usage />
            </section>
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
            <h2 id="stats-heading" class="is-sr-only">{ "Статистика использования" }</h2>
            <nav class="level" aria-label="Статистика">
              <div class="level-item has-text-centered">
                <div>
                  <p class="heading">{ "Total usage" }</p>
                  <p class="title" aria-label="Всего использований: 8">{ 8 }</p>
                </div>
              </div>
              <div class="level-item has-text-centered">
                <div>
                  <p class="heading">{ "On-premise" }</p>
                  <p class="title" aria-label="On-premise установок: 1">{ 1 }</p>
                </div>
              </div>
              <div class="level-item has-text-centered">
                <div>
                  <p class="heading">{ "Followers" }</p>
                  <p class="title" aria-label="Подписчиков: 1 тысяча">{ "1K" }</p>
                </div>
              </div>
              <div class="level-item has-text-centered">
                <div>
                  <p class="heading">{ "Likes" }</p>
                  <p class="title" aria-label="Лайков: 789">{ 789 }</p>
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
            <h2 id="faq-heading" class="title has-text-centered">{ "Часто задаваемые вопросы" }</h2>
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
                    <div class={classes!("box", if is_open { "is-active" } else { "" })} role="article">
                        <button
                            class="content is-medium"
                            onclick={on_click}
                            style="cursor: pointer; background: none; border: none; width: 100%; text-align: left; padding: 0;"
                            aria-expanded={is_open.to_string()}
                            aria-controls={format!("answer-{}", idx)}
                        >
                            <strong>{ &item.question }</strong>
                        </button>
                        <div
                            id={format!("answer-{}", idx)}
                            class="content"
                            style={if is_open { "display: block;" } else { "display: none;" }}
                            role="region"
                            aria-hidden={(!is_open).to_string()}
                        >
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
            <h2 id="examples-heading" class="title has-text-centered">{ "Примеры диалогов" }</h2>
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
        <article class="container" style="border: 1px solid #ccc; padding: 1rem; border-radius: 8px;" role="article">
            <h3 class="title">{ &chat.title }</h3>
            <p class="subtitle">{ &chat.subtitle }</p>

            { for chat.dialogs.iter().enumerate().map(|(idx, dialog)| {
            html! {
            <>
                <div class="block is-flex is-justify-content-flex-end" role="group" aria-label={format!("Сообщение пользователя {}", idx + 1)}>
                    <div class="box" style="max-width: 60%;">
                        <p>{ &dialog.req }</p>
                    </div>
                </div>
                <div class="block is-flex is-justify-content-flex-start" role="group" aria-label={format!("Ответ бота {}", idx + 1)}>
                    <pre class="box" style="margin: 0; white-space: pre-wrap; word-wrap: break-word; overflow-wrap: anywhere;">{ &dialog.res }</pre>
                </div>
            </>
            }

            }) }
        </article>
    }
}

#[function_component(Footer)]
pub(crate) fn footer() -> Html {
    html! {
        <footer class="footer" role="contentinfo">
            <div class="content has-text-centered">
                <p>
                    <small>{ "© 2025 iTmageLAB. All rights reserved." }</small>
                </p>
            </div>
        </footer>
    }
}
