use gloo::net::http::Request;
use leptos::*;

use yaixm::Yaixm;

mod yaixm;

#[component]
fn App() -> impl IntoView {
    let async_data = create_local_resource(|| (), |_| async move { fetch_yaixm().await });

    view! {
        { move || match async_data.get() {
            Some(resource) => match resource {
                Some(yaixm) => view! { <p>"Release date: "{ yaixm.release.airac_date }</p> },
                None => view! {<p>"Error loading YAXIM"</p> }
            }
            None => view! { <p>"Loading YAIXM..."</p> }
        }}
    }
}

// Get YAIXM data from server
async fn fetch_yaixm() -> Option<Yaixm> {
    let result = Request::get("yaixm.json").send().await;
    match result {
        Ok(response) => response.json().await.ok(),
        _ => None,
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}
