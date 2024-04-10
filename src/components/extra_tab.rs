use leptos::*;

use crate::settings::{ExtraType, Settings};

#[component]
pub fn ExtraTab(children: Children, names: Vec<String>, ids: Vec<ExtraType>) -> impl IntoView {
    let setter = use_context::<WriteSignal<Settings>>().expect("to find setter");

    let (get, set) = create_signal(0);

    names
        .iter()
        .zip(children().nodes.iter())
        .zip(ids)
        .enumerate()
        .map(|(n, ((name, child), id))| {
            view! {
                <div class="card block">
                    <header class="card-header is-clickable" on:click=move |_| set(n)>
                        <p class="card-header-title">{ name }</p>
                        <div hidden=move || get()!=n>
                            <div class="card-header-icon">
                                <input class="button is-info is-light is-small ml-2"
                                    type="button" value="Clear"
                                    on:click=move |_| setter.update(|s| s.clear_extra(id)) />
                            </div>
                        </div>
                    </header>

                    <div class="card-content" hidden=move || get()!=n>
                        {child}
                    </div>
                </div>
            }
        })
        .collect_view()
}
