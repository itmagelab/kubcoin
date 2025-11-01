mod button;
mod language_switcher;

use language_switcher::LanguageSwitcher;

use yew::prelude::*;

use crate::i18n::use_language;

#[function_component(Header)]
pub(crate) fn header() -> Html {
    let ctx = use_language();
    let t = &ctx.translations.header;

    html! {
        <header role="banner" class="w-full">
            <div class="flex justify-end py-4 px-4">
                <LanguageSwitcher />
            </div>
            <div class="text-center">
                <div class="flex flex-col lg:flex-row items-center justify-center gap-8 lg:gap-16">
                    <div class="w-full lg:w-1/2">
                        <div class="bg-white rounded-xl shadow-lg p-6 mb-6 transition-all hover-lift">
                            <h1 class="text-4xl md:text-5xl lg:text-6xl font-bold text-gray-900 mb-4 flex items-center justify-center gap-3">
                                <span class="text-4xl">{"🪙"}</span>
                                { "KubCoin" }
                            </h1>
                            <h2 class="text-xl md:text-2xl text-gray-600">{ &t.subtitle }</h2>
                        </div>
                        <nav aria-label={ t.aria_main_actions.clone() } class="space-y-4">
                            <div class="flex flex-col sm:flex-row gap-3 justify-center">
                                <button::Group />
                                <button::Channel />
                            </div>
                            <div class="flex justify-center">
                                <button::Start />
                            </div>
                        </nav>
                    </div>

                    <div class="w-full lg:w-1/2 flex justify-center">
                        <figure class="phone-fade max-w-xs md:max-w-sm lg:max-w-md">
                            <img
                                src="images/IMG_3089.JPG"
                                alt={ t.img_alt_screenshot.clone() }
                                loading="eager"
                                class="w-full h-auto rounded-2xl"
                            />
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
        <div class="max-w-7xl mx-auto">
            <section id="features" aria-labelledby="features-heading" class="mb-16">
                <Features />
            </section>
            <Split />
            <section id="examples" aria-labelledby="examples-heading" class="mb-16">
                <Chats />
            </section>
            <Split />
            <section id="pricing" aria-labelledby="pricing-heading" class="mb-16">
                <Pricing />
            </section>
            <Split />
            <section id="security" aria-labelledby="security-heading" class="mb-16">
                <Security />
            </section>
            <Split />
            <section id="faq" aria-labelledby="faq-heading" class="mb-16">
                <QA />
            </section>
            <Split />
            <section id="statistics" aria-labelledby="stats-heading" class="mb-16">
                <Usage />
            </section>
        </div>
    }
}

#[function_component(Split)]
pub(crate) fn split() -> Html {
    html! {
       <div class="py-8"></div>
    }
}

#[function_component(Features)]
pub(crate) fn features() -> Html {
    let ctx = use_language();
    let features = &ctx.translations.features;
    let ui = &ctx.translations.ui;

    html! {
        <div class="max-w-6xl mx-auto">
            <h2 id="features-heading" class="text-3xl md:text-4xl font-bold text-center text-gray-900 mb-4">
                { &ui.features_section_title }
            </h2>
            <p class="text-lg md:text-xl text-gray-600 text-center mb-12">
                { &ui.features_section_subtitle }
            </p>

            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                { for features.iter().map(|feature| html! {
                    <div class="bg-white rounded-xl shadow-lg p-6 text-center transition-all hover-lift h-full">
                        <div class="text-5xl mb-4">{ &feature.icon }</div>
                        <h3 class="text-xl font-semibold text-gray-900 mb-3">{ &feature.title }</h3>
                        <p class="text-gray-600">{ &feature.description }</p>
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
        <div class="max-w-6xl mx-auto">
            <h2 id="security-heading" class="text-3xl md:text-4xl font-bold text-center text-gray-900 mb-4">
                { &ui.security_section_title }
            </h2>
            <p class="text-lg md:text-xl text-gray-600 text-center mb-12">
                { &ui.security_section_subtitle }
            </p>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                { for security.iter().map(|item| html! {
                    <div class="bg-white rounded-xl shadow-lg p-6">
                        <div class="prose prose-lg">
                            <h3 class="text-xl font-semibold text-gray-900 mb-3">
                                { format!("{} {}", &item.icon, &item.title) }
                            </h3>
                            <p class="text-gray-600">{ &item.description }</p>
                        </div>
                    </div>
                }) }
            </div>

            <div class="bg-blue-50 border border-blue-200 rounded-xl p-6 mt-8">
                <p class="text-center text-blue-800">
                    <strong class="font-semibold">{ &ui.notification_tip }</strong>
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
        <div class="max-w-6xl mx-auto">
            <h2 id="pricing-heading" class="text-3xl md:text-4xl font-bold text-center text-gray-900 mb-4">
                { &ui.pricing_section_title }
            </h2>
            <p class="text-lg md:text-xl text-gray-600 text-center mb-12">
                { &ui.pricing_section_subtitle }
            </p>

            <div class="grid grid-cols-1 lg:grid-cols-2 gap-8 max-w-4xl mx-auto">
                <div class="bg-white rounded-xl shadow-lg p-8 border-2 border-green-500 transition-all hover-lift">
                    <div class="text-center">
                        <div class="text-5xl mb-4">{"🆓"}</div>
                        <h3 class="text-2xl font-bold text-gray-900 mb-2">{ &ui.pricing_free_title }</h3>
                        <p class="text-3xl font-bold text-green-600 mb-1">{ &ui.pricing_free_price }</p>
                        <p class="text-gray-500 text-sm">{ &ui.pricing_free_period }</p>
                    </div>

                    <div class="mt-6">
                        <ul class="space-y-2 text-gray-700">
                            <li class="flex items-center">
                                <span class="text-green-500 mr-2">{"✓"}</span>
                                { &ui.feature_unlimited_operations }
                            </li>
                            <li class="flex items-center">
                                <span class="text-green-500 mr-2">{"✓"}</span>
                                { &ui.feature_all_basic_functions }
                            </li>
                            <li class="flex items-center">
                                <span class="text-green-500 mr-2">{"✓"}</span>
                                { &ui.feature_basic_analytics }
                            </li>
                            <li class="flex items-center">
                                <span class="text-green-500 mr-2">{"✓"}</span>
                                { &ui.feature_expense_categories }
                            </li>
                            <li class="flex items-center">
                                <span class="text-green-500 mr-2">{"✓"}</span>
                                { &ui.feature_math_expressions }
                            </li>
                            <li class="flex items-center">
                                <span class="text-green-500 mr-2">{"✓"}</span>
                                { &ui.feature_monthly_reports }
                            </li>
                            <li class="flex items-center">
                                <span class="text-green-500 mr-2">{"✓"}</span>
                                { &ui.feature_community_support }
                            </li>
                        </ul>
                    </div>

                    <div class="mt-6">
                        <button::Start />
                    </div>
                </div>

                <div class="bg-white rounded-xl shadow-lg p-8 border-2 border-blue-500 transition-all hover-lift relative">
                    <span class="absolute -top-3 right-4 bg-blue-500 text-white px-3 py-1 rounded-full text-sm font-semibold">
                        { &ui.pricing_premium_coming_soon }
                    </span>
                    <div class="text-center">
                        <div class="text-5xl mb-4">{"⭐"}</div>
                        <h3 class="text-2xl font-bold text-gray-900 mb-2">{ &ui.pricing_premium_title }</h3>
                        <p class="text-3xl font-bold text-blue-600 mb-1">{ &ui.pricing_premium_price }</p>
                        <p class="text-gray-500 text-sm">{ &ui.pricing_premium_period }</p>
                    </div>

                    <div class="mt-6">
                        <p class="font-semibold text-gray-900 mb-4">{ &ui.pricing_premium_everything_plus }</p>
                        <ul class="space-y-2 text-gray-700">
                            <li class="flex items-center">
                                <span class="text-blue-500 mr-2">{"✓"}</span>
                                { &ui.feature_advanced_analytics }
                            </li>
                            <li class="flex items-center">
                                <span class="text-blue-500 mr-2">{"✓"}</span>
                                { &ui.feature_charts_visualization }
                            </li>
                            <li class="flex items-center">
                                <span class="text-blue-500 mr-2">{"✓"}</span>
                                { &ui.feature_budget_planning }
                            </li>
                            <li class="flex items-center">
                                <span class="text-blue-500 mr-2">{"✓"}</span>
                                { &ui.feature_multiple_currencies }
                            </li>
                            <li class="flex items-center">
                                <span class="text-blue-500 mr-2">{"✓"}</span>
                                { &ui.feature_goals_savings }
                            </li>
                            <li class="flex items-center">
                                <span class="text-blue-500 mr-2">{"✓"}</span>
                                { &ui.feature_priority_support }
                            </li>
                            <li class="flex items-center">
                                <span class="text-blue-500 mr-2">{"✓"}</span>
                                { &ui.feature_extended_backups }
                            </li>
                            <li class="flex items-center">
                                <span class="text-blue-500 mr-2">{"✓"}</span>
                                { &ui.feature_ai_text_recognition }
                            </li>
                            <li class="flex items-center">
                                <span class="text-blue-500 mr-2">{"✓"}</span>
                                { &ui.feature_ai_voice_recognition}
                            </li>
                        </ul>
                    </div>

                    <button class="w-full mt-6 bg-blue-500 text-white py-3 px-6 rounded-lg font-semibold opacity-50 cursor-not-allowed">
                        { &ui.pricing_premium_in_development }
                    </button>
                </div>
            </div>

            <div class="bg-yellow-50 border border-yellow-200 rounded-xl p-6 mt-8 max-w-4xl mx-auto">
                <p class="text-center text-yellow-800">
                    <strong class="font-semibold">{ "💼 Enterprise: " }</strong>
                    { &ui.pricing_enterprise_text }{ " " }
                    <a href="https://t.me/itmagelab_ru_group" target="_blank" rel="noopener noreferrer" class="text-blue-600 hover:text-blue-800 underline">
                        { &ui.pricing_enterprise_contact }
                    </a>
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
        <div class="max-w-4xl mx-auto">
            <h2 id="stats-heading" class="sr-only">{ &ui.stats_section_title }</h2>
            <div class="grid grid-cols-2 md:grid-cols-4 gap-6" aria-label={ ui.stats_section_title.clone() }>
                <div class="text-center">
                    <p class="text-sm text-gray-500 uppercase tracking-wide">{ &ui.stats_total_usage }</p>
                    <p class="text-3xl font-bold text-gray-900" aria-label={ ui.stats_total_usage_aria.clone() }>{ 8 }</p>
                </div>
                <div class="text-center">
                    <p class="text-sm text-gray-500 uppercase tracking-wide">{ &ui.stats_on_premise }</p>
                    <p class="text-3xl font-bold text-gray-900" aria-label={ ui.stats_on_premise_aria.clone() }>{ 1 }</p>
                </div>
                <div class="text-center">
                    <p class="text-sm text-gray-500 uppercase tracking-wide">{ &ui.stats_followers }</p>
                    <p class="text-3xl font-bold text-gray-900" aria-label={ ui.stats_followers_aria.clone() }>{ "1K" }</p>
                </div>
                <div class="text-center">
                    <p class="text-sm text-gray-500 uppercase tracking-wide">{ &ui.stats_likes }</p>
                    <p class="text-3xl font-bold text-gray-900" aria-label={ ui.stats_likes_aria.clone() }>{ 789 }</p>
                </div>
            </div>
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
        <div class="max-w-4xl mx-auto">
            <h2 id="faq-heading" class="text-3xl md:text-4xl font-bold text-center text-gray-900 mb-8">
                { &ui.faq_section_title }
            </h2>
            <div class="space-y-4">
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
                        <div class={classes!("bg-white", "rounded-xl", "shadow-lg", "p-6", "transition-all", if is_open { "ring-2 ring-blue-500" } else { "" })} role="article">
                            <button
                                class="w-full text-left"
                                onclick={on_click}
                                aria-expanded={is_open.to_string()}
                                aria-controls={format!("answer-{}", idx)}
                            >
                                <div class="flex justify-between items-center">
                                    <strong class="text-lg font-semibold text-gray-900">{ &item.question }</strong>
                                    <span class={classes!("transform", "transition-transform", if is_open { "rotate-180" } else { "" })}>
                                        <svg class="w-5 h-5 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                                        </svg>
                                    </span>
                                </div>
                            </button>
                            <div
                                id={format!("answer-{}", idx)}
                                class={classes!("mt-4", "transition-all", "duration-300", if is_open { "block" } else { "hidden" })}
                                role="region"
                                aria-hidden={(!is_open).to_string()}
                            >
                                <div class="faq-answer">
                                    { &item.answer }
                                </div>
                            </div>
                        </div>
                    }
                }) }
            </div>
        </div>
    }
}

#[function_component(Chats)]
pub(crate) fn chats() -> Html {
    let ctx = use_language();
    let chats = &ctx.translations.chats;
    let ui = &ctx.translations.ui;

    html! {
        <div class="max-w-6xl mx-auto">
            <h2 id="examples-heading" class="text-3xl md:text-4xl font-bold text-center text-gray-900 mb-8">
                { &ui.chats_section_title }
            </h2>
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                { for chats.iter().map(|item| html! {
                    <Chat chat={item.clone()} />
                }) }
            </div>
        </div>
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
        <article class="bg-white rounded-xl shadow-lg p-6 border border-gray-200" role="article">
            <h3 class="text-xl font-semibold text-gray-900 mb-2">{ &chat.title }</h3>
            <p class="text-gray-600 mb-4">{ &chat.subtitle }</p>

            { for chat.dialogs.iter().enumerate().map(|(idx, dialog)| {
            let user_msg = ui.aria_user_message.clone();
            let bot_resp = ui.aria_bot_response.clone();
            html! {
            <div class="space-y-3">
                <div class="flex justify-end" role="group" aria-label={format!("{} {}", user_msg, idx + 1)}>
                    <div class="bg-blue-500 text-white rounded-2xl rounded-br-none px-4 py-2 max-w-[80%]">
                        <p class="text-sm">{ &dialog.req }</p>
                    </div>
                </div>
                <div class="flex justify-start" role="group" aria-label={format!("{} {}", bot_resp, idx + 1)}>
                    <div class="bg-gray-100 text-gray-800 rounded-2xl rounded-bl-none px-4 py-2 max-w-[80%]">
                        <pre class="text-sm whitespace-pre-wrap break-words overflow-wrap-anywhere">{ &dialog.res }</pre>
                    </div>
                </div>
            </div>
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
        <footer class="bg-gray-800 text-white py-8" role="contentinfo">
            <div class="max-w-6xl mx-auto px-4 text-center">
                <p class="text-sm text-gray-300">
                    <small>{ &footer.copyright }</small>
                </p>
                <p class="text-sm text-gray-300 mt-2">
                    <small>{ &footer.developed_by }</small>
                </p>
            </div>
        </footer>
    }
}
