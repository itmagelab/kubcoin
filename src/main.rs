mod handler;
mod html;
mod i18n;

use tracing_subscriber::{
    fmt::format::{FmtSpan, Pretty},
    prelude::*,
};

use wasm_bindgen::JsValue;
use yew::prelude::*;

// https://core.telegram.org/bots/webapps#initializing-mini-apps
// https://bulma.io/documentation/

#[function_component(App)]
fn app() -> Html {
    html! {
        <i18n::LanguageProvider>
            <AppContent />
        </i18n::LanguageProvider>
    }
}

#[function_component(AppContent)]
fn app_content() -> Html {
    html! {
        <>
        <section class="hero">
            <div class="hero-body">
                <html::Header />
            </div>
        </section>
        <section class="section">
            <html::Body />
        </section>
        <html::Footer />
        </>
    }
}

fn main() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_writer(tracing_web::MakeConsoleWriter)
        .without_time()
        .with_span_events(FmtSpan::ACTIVE);
    let perf_layer = tracing_web::performance_layer().with_details_from_fields(Pretty::default());
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .init();
    let object = JsValue::from("Starting app...");
    tracing::info!("{}", object.as_string().unwrap());

    yew::Renderer::<App>::new().render();
}
