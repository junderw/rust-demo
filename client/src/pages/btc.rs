use crate::Page;
use api_boundary::BtcInfo;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Btc(btc_info: Signal<Option<BtcInfo>>) -> impl IntoView {
    view! {
        <h2>"Getting BTC Price Info"</h2>
        {move || match btc_info.get() {
            Some(info) => {
                view! { <p>"Well... " {info.price_info} "..."</p> }
                    .into_view()
            }
            None => {
                view! {
                    <p>"You are not logged in."</p>
                    <A href=Page::Login.path()>"Login now."</A>
                }
                    .into_view()
            }
        }}
    }
}
