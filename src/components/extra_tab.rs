use leptos::*;

use crate::settings::{ExtraType, Settings};

#[component]
pub fn ExtraTab(children: Children, names: Vec<String>, ids: Vec<ExtraType>) -> impl IntoView {
    let setter = use_context::<WriteSignal<Settings>>().expect("to find setter");

    names
        .iter()
        .zip(children().nodes.iter())
        .zip(ids)
        .map(|((name, child), id)| {
            view! {
                <div class="card block">
                    <header class="card-header">
                        <p class="card-header-title">{ name }</p>
                        <div class="card-header-icon">
                            <input class="button is-info is-light is-small ml-2" type="button" value="Clear" on:click=move |_| setter.update(|s| s.clear_extra(id)) />
                        </div>
                    </header>

                    <div class="card-content">
                        {child}
                    </div>
                </div>
            }
        })
        .collect_view()
}
