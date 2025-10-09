mod button;

use yew::prelude::*;

struct QAItem {
    question: &'static str,
    answer: &'static str,
}

pub(crate) struct ChatItem {
    title: &'static str,
    subtitle: &'static str,
    dialog: Vec<(String, String)>,
}

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
    let items = [
        QAItem {
            question: "Что такое KubCoin?",
            answer: "KubCoin — это умный помощник для управления личными финансами. Он помогает контролировать расходы, планировать бюджет и анализировать траты.",
        },
        QAItem {
            question: "Можно ли запустить KubCoin на своём сервере?",
            answer: "Да! Вы можете развернуть KubCoin локально (on-premise) и полностью контролировать хранение данных. Ваша приватность — только в ваших руках.",
        },
    ];

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
                            <strong>{ item.question }</strong>
                        </div>
                        <div class="content" style={if is_open { "display: block;" } else { "display: none;" }}>
                            { item.answer }
                        </div>
                    </div>
                }
            }) }
        </div>
    }
}

#[function_component(Chats)]
pub(crate) fn chats() -> Html {
    let items = [
        ChatItem {
            title: "Добавляйте свои расходы",
            subtitle: "KubCoin добавит операцию в базу данных",
            dialog: vec![
                (
                    String::from("/start"),
                    String::from(
                        r#"
✅ Всё готово!

Теперь можно добавлять записи, вносить изменения и вести учёт вместе! 🚀

🔧 Примеры команд:
-100 еда // покупка продуктов
1000-560 работа
-1500 другое или просто -1500
100+150-10 или $ 100/3 + 500 - (123+67)
/report или /report 5 — отчёт за текущий или 5-й месяц

Введите /help, чтобы узнать больше о доступных командах и возможностях бота.
"#,
                    ),
                ),
                (
                    String::from("-1540 Еда // Обед"),
                    String::from(
                        r#"
📆 День/Месяц: 450₽ / 100350₽
( -1540 Еда )

✓✅ Операция добавлена!
Идентификатор: 358f9cb4
Контрольная сумма: 689aa259
"#,
                    ),
                ),
            ],
        },
        ChatItem {
            title: "Используйте сокращенные команды",
            subtitle: "KubCoin умеет повторять последнюю операцию с помощью `!`",
            dialog: vec![
                (
                    String::from("!"),
                    String::from(
                        r#"
📆 День/Месяц: -1090₽ / 98810₽
( -1540 Еда )

✓✅ Операция добавлена!
Идентификатор: c70ee2a9
Контрольная сумма: 0fb31b86
"#,
                    ),
                ),
                (
                    String::from("-500"),
                    String::from(
                        r#"
📆 День/Месяц: -1590₽ / 98310₽
( -500 Еда )

✓✅ Операция добавлена!
Идентификатор: 81cac754
Контрольная сумма: c3a507a1
"#,
                    ),
                ),
            ],
        },
        ChatItem {
            title: "Выводи отчет за месяц",
            subtitle: "KubCoin умеет выводить отчет по месяцам и датам",
            dialog: vec![(
                String::from("/report"),
                String::from(
                    r#"
📈 Доходы за 10/2025
Работа: 440₽
Заработная плата: 10000₽
---

Итого: 10440₽

📉 Расходы за 10/2025
Еда: 1600₽
Другое: 1500₽
---

Итого: 3100₽

Остаток: 7340₽
"#,
                ),
            )],
        },
        ChatItem {
            title: "Используй ИИ для операций",
            subtitle: "KubCoin умеет анализировать текст и голосовые",
            dialog: vec![
                (
                    String::from("# Лена купила у меня на 1000 рублей цветы"),
                    String::from(
                        r#"
📆 День/Месяц: 1000₽ / 1000₽
( +1000 Другое )

✓✅ Операция добавлена!
Идентификатор: d9b710cd
Контрольная сумма: faf35c78
"#,
                    ),
                ),
                (
                    String::from("< ГОЛОСОВОЕ СООБЩЕНИЕ >"),
                    String::from(
                        r#"
📆 День/Месяц: 2000₽ / 2000₽
( +1000 Другое )

✓✅ Операция добавлена!
Идентификатор: b776a48b
Контрольная сумма: f7da958d
"#,
                    ),
                ),
            ],
        },
        ChatItem {
            title: "Управление учетной книгой",
            subtitle: "KubCoin может удалить все ваши операции",
            dialog: vec![
                (
                    String::from("/unregister"),
                    String::from(
                        r#"
❌ Авторизация отменена.

Вы в любой момент можете начать регистрацию заново,
если захотите получить доступ к функциям бота.

Просто введите:
/register
"#,
                    ),
                ),
                (
                    String::from("/register"),
                    String::from(
                        r#"
✅ Вы успешно авторизованы!

📘 Книга учёта успешно привязана к чату.
Теперь вы можете использовать все доступные команды.

Начните с:
/help — чтобы увидеть список возможностей бота.
или /start — чтобы сразу начать вводить операции.
"#,
                    ),
                ),
            ],
        },
        ChatItem {
            title: "Дополнительные команды",
            subtitle: "KubCoin может получать новые команды в процессе обновления",
            dialog: vec![
                (
                    String::from("/support"),
                    String::from(
                        r#"
📬 Поддержка и обратная связь
Если заметили ошибку, есть идея или просто хотите что-то обсудить — пишите:

✉️ Почта: <i@itmage.ru>
💬 Telegram: @iTmagelab или Канал: @iTmagelab_ru

Всегда рад обратной связи!
"#,
                    ),
                ),
                (
                    String::from("/donate"),
                    String::from(
                        r#"
Поддержка проекта
Спасибо, что поддерживаете развитие проекта! ❤️
"#,
                    ),
                ),
            ],
        },
    ];
    html! {
        <>
            { for items.chunks(2).map(|pair| html! {
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
            <div class="title"><p>{ chat.title }</p></div>
            <div class="subtitle"><p>{ chat.subtitle }</p></div>

            { for chat.dialog.iter().map(|(req, ans)| {
            html! {
            <>
                <div class="block is-flex is-justify-content-flex-end">
                    <div class="box" style="max-width: 60%;">
                        <p>{ req }</p>
                    </div>
                </div>
                <div class="block is-flex is-justify-content-flex-start">
                    <pre class="box" style="margin: 0; white-space: pre-wrap; word-wrap: break-word; overflow-wrap: anywhere;">{ ans }</pre>
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
