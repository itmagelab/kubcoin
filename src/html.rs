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
            <section id="features" aria-labelledby="features-heading">
                <Features />
            </section>
            <Split />
            <section id="examples" aria-labelledby="examples-heading">
                <Chats />
            </section>
            <Split />
            <section id="pricing" aria-labelledby="pricing-heading">
                <Pricing />
            </section>
            <Split />
            <section id="security" aria-labelledby="security-heading">
                <Security />
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

#[function_component(Features)]
pub(crate) fn features() -> Html {
    html! {
        <div class="container">
            <h2 id="features-heading" class="title is-2 has-text-centered">{ "Возможности" }</h2>
            <p class="subtitle has-text-centered has-text-grey">{ "Всё необходимое для управления финансами" }</p>

            <div class="columns is-multiline" style="margin-top: 2rem;">
                <div class="column is-half-tablet is-one-third-desktop">
                    <div class="box has-text-centered" style="height: 100%;">
                        <span class="icon is-large" style="font-size: 3rem;">{ "🤖" }</span>
                        <h3 class="title is-4">{ "AI-помощник" }</h3>
                        <p>{ "Понимает естественный язык и голосовые сообщения. Просто скажите что потратили — бот сам всё поймёт." }</p>
                    </div>
                </div>

                <div class="column is-half-tablet is-one-third-desktop">
                    <div class="box has-text-centered" style="height: 100%;">
                        <span class="icon is-large" style="font-size: 3rem;">{ "📊" }</span>
                        <h3 class="title is-4">{ "Аналитика" }</h3>
                        <p>{ "Детальные отчёты по категориям, месяцам и периодам. Видите куда уходят деньги." }</p>
                    </div>
                </div>

                <div class="column is-half-tablet is-one-third-desktop">
                    <div class="box has-text-centered" style="height: 100%;">
                        <span class="icon is-large" style="font-size: 3rem;">{ "🔐" }</span>
                        <h3 class="title is-4">{ "Безопасность" }</h3>
                        <p>{ "Написан на Rust для максимальной надёжности. Поддержка on-premise для полного контроля данных." }</p>
                    </div>
                </div>

                <div class="column is-half-tablet is-one-third-desktop">
                    <div class="box has-text-centered" style="height: 100%;">
                        <span class="icon is-large" style="font-size: 3rem;">{ "⚡" }</span>
                        <h3 class="title is-4">{ "Быстрый учёт" }</h3>
                        <p>{ "Добавляйте операции за секунды. Математические выражения, повтор последней операции — всё для удобства." }</p>
                    </div>
                </div>

                <div class="column is-half-tablet is-one-third-desktop">
                    <div class="box has-text-centered" style="height: 100%;">
                        <span class="icon is-large" style="font-size: 3rem;">{ "👥" }</span>
                        <h3 class="title is-4">{ "Совместный бюджет" }</h3>
                        <p>{ "Используйте бота в группах для ведения семейного или проектного бюджета вместе." }</p>
                    </div>
                </div>

                <div class="column is-half-tablet is-one-third-desktop">
                    <div class="box has-text-centered" style="height: 100%;">
                        <span class="icon is-large" style="font-size: 3rem;">{ "🎯" }</span>
                        <h3 class="title is-4">{ "Категории" }</h3>
                        <p>{ "Автоматическая категоризация расходов. Гибкая настройка своих категорий." }</p>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[function_component(Security)]
pub(crate) fn security() -> Html {
    html! {
        <div class="container">
            <h2 id="security-heading" class="title is-2 has-text-centered">{ "Безопасность и приватность" }</h2>
            <p class="subtitle has-text-centered has-text-grey">{ "Ваши данные под надёжной защитой" }</p>

            <div class="columns is-vcentered" style="margin-top: 2rem;">
                <div class="column is-half">
                    <div class="content">
                        <h3 class="title is-4">{ "🔒 Защита данных" }</h3>
                        <p>{ "Ваши финансовые данные надежно защищены. Никто, кроме вас, не имеет доступа к вашим операциям." }</p>

                        <h3 class="title is-4">{ "🏠 On-premise решение" }</h3>
                        <p>{ "Хотите полный контроль? Разверните KubCoin на своём сервере. Ваши данные остаются только у вас — без облаков, без третьих сторон." }</p>

                        <h3 class="title is-4">{ "🦀 Написано на Rust" }</h3>
                        <p>{ "Rust гарантирует безопасность памяти и отсутствие уязвимостей на уровне кода. Тот же язык, который используется в критических системах и блокчейн-проектах." }</p>
                    </div>
                </div>

                <div class="column is-half">
                    <div class="content">
                        <h3 class="title is-4">{ "🚫 Никаких персональных данных" }</h3>
                        <p>{ "Не требуем email, пароли или другие личные данные. Только Telegram ID для идентификации." }</p>

                        <h3 class="title is-4">{ "🔄 Резервные копии" }</h3>
                        <p>{ "Автоматическое создание backup'ов. Ваши данные никогда не потеряются." }</p>

                        <h3 class="title is-4">{ "✅ Open Source" }</h3>
                        <p>{ "Планируется открытие исходного кода. Вы сможете самостоятельно проверить безопасность и внести свои улучшения." }</p>
                    </div>
                </div>
            </div>

            <div class="notification is-info is-light" style="margin-top: 2rem;">
                <p class="has-text-centered">
                    <strong>{ "💡 Совет:" }</strong>
                    { " Для максимальной безопасности используйте on-premise версию KubCoin на своём сервере или VPS." }
                </p>
            </div>
        </div>
    }
}

#[function_component(Pricing)]
pub(crate) fn pricing() -> Html {
    html! {
        <div class="container">
            <h2 id="pricing-heading" class="title is-2 has-text-centered">{ "Тарифы" }</h2>
            <p class="subtitle has-text-centered has-text-grey">{ "Выберите подходящий план" }</p>

            <div class="columns is-centered" style="margin-top: 2rem;">
                <div class="column is-half-tablet is-one-third-desktop">
                    <div class="box" style="border: 2px solid #48c774;">
                        <div class="has-text-centered">
                            <span class="icon is-large" style="font-size: 3rem;">{ "🆓" }</span>
                            <h3 class="title is-3">{ "Бесплатный" }</h3>
                            <p class="title is-4 has-text-success">{ "0 ₽" }</p>
                            <p class="subtitle is-6 has-text-grey">{ "навсегда" }</p>
                        </div>

                        <div class="content">
                            <ul style="list-style: none; padding-left: 0;">
                                <li>{ "✅ Неограниченные операции" }</li>
                                <li>{ "✅ Все основные функции" }</li>
                                <li>{ "✅ Базовая аналитика" }</li>
                                <li>{ "✅ Категории расходов" }</li>
                                <li>{ "✅ Математические выражения" }</li>
                                <li>{ "✅ AI распознавание текста" }</li>
                                <li>{ "✅ Месячные отчёты" }</li>
                                <li>{ "✅ Поддержка сообщества" }</li>
                            </ul>
                        </div>

                        <button::Start />
                    </div>
                </div>

                <div class="column is-half-tablet is-one-third-desktop">
                    <div class="box" style="border: 2px solid #3273dc; position: relative;">
                        <span class="tag is-primary" style="position: absolute; top: -10px; right: 20px;">{ "Скоро" }</span>
                        <div class="has-text-centered">
                            <span class="icon is-large" style="font-size: 3rem;">{ "⭐" }</span>
                            <h3 class="title is-3">{ "Premium" }</h3>
                            <p class="title is-4 has-text-primary">{ "299 ₽" }</p>
                            <p class="subtitle is-6 has-text-grey">{ "в месяц" }</p>
                        </div>

                        <div class="content">
                            <p><strong>{ "Всё из бесплатного, плюс:" }</strong></p>
                            <ul style="list-style: none; padding-left: 0;">
                                <li>{ "✅ Расширенная аналитика" }</li>
                                <li>{ "✅ Экспорт в Excel/CSV" }</li>
                                <li>{ "✅ Графики и визуализация" }</li>
                                <li>{ "✅ Планирование бюджета" }</li>
                                <li>{ "✅ Множественные валюты" }</li>
                                <li>{ "✅ Цели и накопления" }</li>
                                <li>{ "✅ Приоритетная поддержка" }</li>
                                <li>{ "✅ Расширенные backup" }</li>
                            </ul>
                        </div>

                        <button class="button is-primary is-fullwidth" disabled=true>
                            { "В разработке" }
                        </button>
                    </div>
                </div>
            </div>

            <div class="notification is-warning is-light" style="margin-top: 2rem;">
                <p class="has-text-centered">
                    <strong>{ "💼 Enterprise:" }</strong>
                    { " Нужно on-premise решение для команды или компании? " }
                    <a href="https://t.me/itmagelab_ru_group" target="_blank" rel="noopener noreferrer">{ "Свяжитесь с нами" }</a>
                    { " для индивидуального предложения." }
                </p>
            </div>
        </div>
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
