// Copyright 2024, Alan Sparrow
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or (at
// your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
//
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
