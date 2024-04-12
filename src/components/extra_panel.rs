use leptos::*;

use crate::settings::{ExtraType, Settings};

#[component]
pub fn ExtraPanel(names: Vec<String>, id: ExtraType) -> impl IntoView {
    let setter = use_context::<WriteSignal<Settings>>().expect("to find setter");
    let getter = use_context::<ReadSignal<Settings>>().expect("to find getter");

    view! {
        <div class="columns is-multiline">
            {names
                .into_iter()
                .map(|n| {
                    let nc1 = n.clone();
                    let nc2 = n.clone();
                    view! {
                        <div class="column is-one-third">
                            <div class="field">
                                <label class="checkbox">
                                    <input
                                        type="checkbox"
                                        class="mr-2"
                                        prop:checked=move || {
                                            getter.with(|s| s.get_extra(id).contains(&nc1))
                                        }
                                        on:input=move |ev| {
                                            setter
                                                .update(|s| {
                                                    s.set_extra(id, nc2.clone(), event_target_checked(&ev))
                                                })
                                        }
                                    />
                                    {&n}
                                </label>
                            </div>
                        </div>
                    }
                })
                .collect_view()}

        </div>
    }
}
