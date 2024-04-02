use gloo::file::{Blob, ObjectUrl};
use gloo::net::http::Request;
use leptos::*;
use std::collections::HashSet;

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
    view! {
        <div class="container block">
            <AirspaceView on_change=|(a, b)| {logging::log!("{}, {}", a, b)}/>

            <RatView rat_names=rat_names(&yaixm) on_change=|x| {logging::log! ("{:?}", x)}/>

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
                    }>
                    {"Get Airspace"}
                </button>
                </div>
            </div>
        </div>
    }
}

#[component]
fn AirspaceView(#[prop(into)] on_change: Callback<(String, String)>) -> impl IntoView {
    view! {
        <div class="columns">
            <div class="column is-one-third">
            <div class="field">
                <label class="label">
                {"ATZ"}
                <div class="control">
                    <div class="select is-fullwidth" on:change=move |ev| {
                        on_change(("atz".to_string(), event_target_value(&ev)))
            }>
                    <select name="atz">
                        <option value="classd">{"Class D"}</option>
                        <option value= "ctr">{"Control Zone"}</option>
                    </select>
                    </div>
                </div>
                </label>
            </div>
            </div>
        </div>
    }
}

#[component]
fn RatView(
    rat_names: Vec<String>,
    #[prop(into)] on_change: Callback<HashSet<String>>,
) -> impl IntoView {
    let (get, set) = create_signal(HashSet::<String>::new());
    view! {
        <div>
            <ul>
                { rat_names.into_iter()
                    .map(|n| {
                        let nx = n.clone();
                        let nz = n.clone();
                        let checked = move || get().contains(&nz);
                        view! {
                            <div class="field">
                            <label class="checkbox">
                            <input name={&n} type="checkbox" class="mr-2" prop:checked={checked} on:change = move |ev| {
                                if event_target_checked(&ev) {
                                    set.update(|s| { s.insert(nx.clone()); });
                                } else {
                                    set.update(|s| { s.remove(&nx); });
                                }
                                on_change(get())
                            }/>
                            {&n}
                            </label>
                            </div>
                        }
                    })
                    .collect_view()
                }
            </ul>
            <button on:click = move |_| { set.update(|s|  s.clear()); on_change(get()) }>Clear</button>
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
