use gloo::file::{Blob, ObjectUrl};
use gloo::net::http::Request;
use leptos::*;

use yaixm::{rat_names, Yaixm};

mod yaixm;

#[component]
fn App() -> impl IntoView {
    let async_data = create_local_resource(|| (), |_| async move { fetch_yaixm().await });

    view! {
        { move || match async_data.get() {
            Some(resource) => match resource {
                Some(yaixm) => {
                    view! { <MainView yaixm=yaixm/> }.into_view()
                },
                None => view! { <p>"Error loading YAXIM"</p> }.into_view()
            }
            None => view! { <p>"Loading YAIXM, please wait..."</p> }.into_view()
        }}
    }
}

#[component]
fn MainView(yaixm: Yaixm) -> impl IntoView {
    let rat_names = rat_names(&yaixm);
    view! {
        <div class="container block">
            <div class="columns">
                <div class="column is-one-third">
                <div class="field">
                    <label class="label">
                    {"ATZ"}
                    <div class="control">
                        <div class="select is-fullwidth">
                        <select name="atz">
                            <option>{"Class D"}</option>
                            <option>{"Control Zone"}</option>
                        </select>
                        </div>
                    </div>
                    </label>
                </div>
                </div>
            </div>

            <div>
                <ul>
                    { rat_names.into_iter()
                        .map(|n| view! {
                            <div class="field">
                            <label class="checkbox">
                            <input name={n.clone()} type="checkbox" class="mr-2"/>
                            {n}
                            </label>
                            </div>
                        })
                        .collect_view()
                    }
                </ul>
            </div>

            <div class="container block">
                <div class="mx-4">
                <button type="submit" class="button is-primary"
                    on:click = |_| {
                        let blob = Blob::new("Hello Alan");
                        let object_url = ObjectUrl::from(blob);

                        let a = leptos::html::a();
                        a.set_download("openair.txt");
                        a.set_href(&object_url);
                        a.click();
                    }
                >
                    {"Get Airspace"}
                </button>
                </div>
            </div>
        </div>
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
