mod html;
mod i18n;
mod router;

use router::Route;
use tracing_subscriber::{
    fmt::format::{FmtSpan, Pretty},
    prelude::*,
};

use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(route: Route) -> Html {
    let language = route.to_language();

    // Handle root path redirection
    if route == Route::Home {
        let target_route = Route::from_language(language);
        return html! {
            <Redirect<Route> to={target_route} />
        };
    }

    html! {
        <i18n::LanguageProvider initial_language={language}>
            <AppContent />
        </i18n::LanguageProvider>
    }
}

#[function_component(AppContent)]
fn app_content() -> Html {
    html! {
        <div class="min-h-screen bg-gradient-to-br from-green-50 to-emerald-100">
            <section class="py-8 md:py-16">
                <div class="container mx-auto px-4">
                    <html::Header />
                </div>
            </section>
            <section class="py-8 md:py-16">
                <div class="container mx-auto px-4">
                    <html::Body />
                </div>
            </section>
            <html::Footer />
        </div>
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
