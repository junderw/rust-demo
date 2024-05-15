use crate::Page;
use leptos::*;
use leptos_router::*;

#[component]
pub fn NavBar(
    logged_in: Signal<bool>,
    #[prop(into)] on_logout: Callback<()>,
    #[prop(into)] btc_price: Callback<()>,
) -> impl IntoView {
    view! {
        <nav>
            <Show
                when=move || logged_in.get()
                fallback=|| {
                    view! {
                        <A href=Page::Login.path()>"Login"</A>
                        " | "
                        <A href=Page::Register.path()>"Register"</A>
                    }
                }
            >
                <a
                    href="#"
                    on:click=move |_| on_logout.call(())
                >
                    "Logout"
                </a>
                " | "
                <a href="/">
                    "Go Home"
                </a>
                " | "
                <a
                    href="#"
                    on:click=move |_| btc_price.call(())
                >
                    "Show BTC Price"
                </a>
            </Show>
        </nav>
    }
}
