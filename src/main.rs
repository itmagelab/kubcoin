mod handler;
mod html;

use yew::prelude::*;

// https://core.telegram.org/bots/webapps#initializing-mini-apps
// https://bulma.io/documentation/

#[function_component(App)]
fn app() -> Html {
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
    yew::Renderer::<App>::new().render();
}
