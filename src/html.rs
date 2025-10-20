mod button;
mod language_switcher;

pub(crate) use language_switcher::LanguageSwitcher;

use yew::prelude::*;

use crate::i18n::use_language;

#[function_component(Header)]
pub(crate) fn header() -> Html {
    let ctx = use_language();
    let t = &ctx.translations.header;

    html! {
        <header role="banner">
            <div class="container">
              <div style="display: flex; justify-content: flex-end; padding: 1rem 0;">
                <LanguageSwitcher />
              </div>
              <div class="has-text-centered">
                <div class="columns is-vcentered is-multiline">
                  <div class="column is-full-mobile is-half-tablet">
                    <div class="box">
                      <h1 class="title is-1 has-text-weight-bold">
                        <span class="icon is-large"><i class="fas fa-coins"></i></span>{ "KubCoin" }
                      </h1>
                      <h2 class="subtitle is-3 has-text-grey">{ &t.subtitle }</h2>
                    </div>
                    <nav aria-label={ t.aria_main_actions.clone() }>
                        <div class="buttons is-centered" style="margin-bottom: 0.5rem;">
                            <button::Group />
                            <button::Channel />
                        </div>
                        <div class="buttons is-centered">
                            <button::Start />
                        </div>
                    </nav>
                  </div>

                  <div class="column is-full-mobile is-half-tablet">
                    <figure class="image phone-fade" style="max-width: 300px; margin: 0 auto;">
                      <img src="images/IMG_3089.JPG" alt={ t.img_alt_screenshot.clone() } loading="eager" />
                    </figure>
                  </div>
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
    let ctx = use_language();
    let features = &ctx.translations.features;
    let ui = &ctx.translations.ui;

    html! {
        <div class="container">
            <h2 id="features-heading" class="title is-2 has-text-centered">{ &ui.features_section_title }</h2>
            <p class="subtitle has-text-centered has-text-grey">{ &ui.features_section_subtitle }</p>

            <div class="columns is-multiline" style="margin-top: 2rem;">
                { for features.iter().map(|feature| html! {
                    <div class="column is-half-tablet is-one-third-desktop">
                        <div class="box has-text-centered" style="height: 100%;">
                            <span class="icon is-large" style="font-size: 3rem;">{ &feature.icon }</span>
                            <h3 class="title is-4">{ &feature.title }</h3>
                            <p>{ &feature.description }</p>
                        </div>
                    </div>
                }) }
            </div>
        </div>
    }
}

#[function_component(Security)]
pub(crate) fn security() -> Html {
    let ctx = use_language();
    let security = &ctx.translations.security;
    let ui = &ctx.translations.ui;

    html! {
        <div class="container">
            <h2 id="security-heading" class="title is-2 has-text-centered">{ &ui.security_section_title }</h2>
            <p class="subtitle has-text-centered has-text-grey">{ &ui.security_section_subtitle }</p>

            <div class="columns is-multiline" style="margin-top: 2rem;">
                { for security.iter().map(|item| html! {
                    <div class="column is-half">
                        <div class="content">
                            <h3 class="title is-4">{ format!("{} {}", &item.icon, &item.title) }</h3>
                            <p>{ &item.description }</p>
                        </div>
                    </div>
                }) }
            </div>

            <div class="notification is-info is-light" style="margin-top: 2rem;">
                <p class="has-text-centered">
                    <strong>{ &ui.notification_tip }</strong>
                    { " " }{ &ui.notification_tip_text }
                </p>
            </div>
        </div>
    }
}

#[function_component(Pricing)]
pub(crate) fn pricing() -> Html {
    let ctx = use_language();
    let ui = &ctx.translations.ui;

    html! {
        <div class="container">
            <h2 id="pricing-heading" class="title is-2 has-text-centered">{ &ui.pricing_section_title }</h2>
            <p class="subtitle has-text-centered has-text-grey">{ &ui.pricing_section_subtitle }</p>

            <div class="columns is-centered" style="margin-top: 2rem;">
                <div class="column is-half-tablet is-one-third-desktop">
                    <div class="box" style="border: 2px solid #48c774;">
                        <div class="has-text-centered">
                            <span class="icon is-large" style="font-size: 3rem;">{ "🆓" }</span>
                            <h3 class="title is-3">{ &ui.pricing_free_title }</h3>
                            <p class="title is-4 has-text-success">{ &ui.pricing_free_price }</p>
                            <p class="subtitle is-6 has-text-grey">{ &ui.pricing_free_period }</p>
                        </div>

                        <div class="content">
                            <ul style="list-style: none; padding-left: 0;">
                                <li>{ &ui.feature_unlimited_operations }</li>
                                <li>{ &ui.feature_all_basic_functions }</li>
                                <li>{ &ui.feature_basic_analytics }</li>
                                <li>{ &ui.feature_expense_categories }</li>
                                <li>{ &ui.feature_math_expressions }</li>
                                <li>{ &ui.feature_ai_text_recognition }</li>
                                <li>{ &ui.feature_monthly_reports }</li>
                                <li>{ &ui.feature_community_support }</li>
                            </ul>
                        </div>

                        <button::Start />
                    </div>
                </div>

                <div class="column is-half-tablet is-one-third-desktop">
                    <div class="box" style="border: 2px solid #3273dc; position: relative;">
                        <span class="tag is-primary" style="position: absolute; top: -10px; right: 20px;">{ &ui.pricing_premium_coming_soon }</span>
                        <div class="has-text-centered">
                            <span class="icon is-large" style="font-size: 3rem;">{ "⭐" }</span>
                            <h3 class="title is-3">{ &ui.pricing_premium_title }</h3>
                            <p class="title is-4 has-text-primary">{ &ui.pricing_premium_price }</p>
                            <p class="subtitle is-6 has-text-grey">{ &ui.pricing_premium_period }</p>
                        </div>

                        <div class="content">
                            <p><strong>{ &ui.pricing_premium_everything_plus }</strong></p>
                            <ul style="list-style: none; padding-left: 0;">
                                <li>{ &ui.feature_advanced_analytics }</li>
                                <li>{ &ui.feature_charts_visualization }</li>
                                <li>{ &ui.feature_budget_planning }</li>
                                <li>{ &ui.feature_multiple_currencies }</li>
                                <li>{ &ui.feature_goals_savings }</li>
                                <li>{ &ui.feature_priority_support }</li>
                                <li>{ &ui.feature_extended_backups }</li>
                            </ul>
                        </div>

                        <button class="button is-primary is-fullwidth" disabled=true>
                            { &ui.pricing_premium_in_development }
                        </button>
                    </div>
                </div>
            </div>

            <div class="notification is-warning is-light" style="margin-top: 2rem;">
                <p class="has-text-centered">
                    <strong>{ "💼 Enterprise: " }</strong>
                    { &ui.pricing_enterprise_text }{ " " }
                    <a href="https://t.me/itmagelab_ru_group" target="_blank" rel="noopener noreferrer">{ &ui.pricing_enterprise_contact }</a>
                    { " " }{ &ui.pricing_enterprise_suffix }
                </p>
            </div>
        </div>
    }
}

#[function_component(Usage)]
pub(crate) fn usage() -> Html {
    let ctx = use_language();
    let ui = &ctx.translations.ui;

    html! {
        <div class="container">
            <h2 id="stats-heading" class="is-sr-only">{ &ui.stats_section_title }</h2>
            <nav class="level" aria-label={ ui.stats_section_title.clone() }>
              <div class="level-item has-text-centered">
                <div>
                  <p class="heading">{ &ui.stats_total_usage }</p>
                  <p class="title" aria-label={ ui.stats_total_usage_aria.clone() }>{ 8 }</p>
                </div>
              </div>
              <div class="level-item has-text-centered">
                <div>
                  <p class="heading">{ &ui.stats_on_premise }</p>
                  <p class="title" aria-label={ ui.stats_on_premise_aria.clone() }>{ 1 }</p>
                </div>
              </div>
              <div class="level-item has-text-centered">
                <div>
                  <p class="heading">{ &ui.stats_followers }</p>
                  <p class="title" aria-label={ ui.stats_followers_aria.clone() }>{ "1K" }</p>
                </div>
              </div>
              <div class="level-item has-text-centered">
                <div>
                  <p class="heading">{ &ui.stats_likes }</p>
                  <p class="title" aria-label={ ui.stats_likes_aria.clone() }>{ 789 }</p>
                </div>
              </div>
            </nav>
        </div>
    }
}

#[function_component(QA)]
fn qa() -> Html {
    let ctx = use_language();
    let items = &ctx.translations.qa;
    let ui = &ctx.translations.ui;

    let open_index = use_state(|| None);

    html! {
        <div class="container">
            <h2 id="faq-heading" class="title has-text-centered">{ &ui.faq_section_title }</h2>
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
    let ctx = use_language();
    let chats = &ctx.translations.chats;
    let ui = &ctx.translations.ui;

    html! {
        <>
            <h2 id="examples-heading" class="title has-text-centered">{ &ui.chats_section_title }</h2>
            { for chats.chunks(2).map(|pair| html! {
                <div class="columns is-multiline">
                    <div class="column">
                    </div>
                    { for pair.iter().map(|item| html! {
                        <div class="column is-one-third">
                            <Chat chat={item.clone()} />
                        </div>
                    }) }
                    <div class="column">
                    </div>
                </div>
            }) }
        </>
    }
}

#[derive(Properties, PartialEq)]
pub(crate) struct ChatProps {
    pub(crate) chat: crate::i18n::ChatTranslation,
}

#[function_component(Chat)]
pub(crate) fn chat(props: &ChatProps) -> Html {
    let ctx = use_language();
    let ui = &ctx.translations.ui;
    let chat = &props.chat;

    html! {
        <article class="container" style="border: 1px solid #ccc; padding: 1rem; border-radius: 8px;" role="article">
            <h3 class="title">{ &chat.title }</h3>
            <p class="subtitle">{ &chat.subtitle }</p>

            { for chat.dialogs.iter().enumerate().map(|(idx, dialog)| {
            let user_msg = ui.aria_user_message.clone();
            let bot_resp = ui.aria_bot_response.clone();
            html! {
            <>
                <div class="block is-flex is-justify-content-flex-end" role="group" aria-label={format!("{} {}", user_msg, idx + 1)}>
                    <div class="box" style="max-width: 60%;">
                        <p>{ &dialog.req }</p>
                    </div>
                </div>
                <div class="block is-flex is-justify-content-flex-start" role="group" aria-label={format!("{} {}", bot_resp, idx + 1)}>
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
    let ctx = use_language();
    let footer = &ctx.translations.footer;

    html! {
        <footer class="footer" role="contentinfo">
            <div class="content has-text-centered">
                <p>
                    <small>{ &footer.copyright }</small>
                </p>
                <p>
                    <small>{ &footer.developed_by }</small>
                </p>
            </div>
        </footer>
    }
}
