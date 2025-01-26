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
use leptos::prelude::*;

use crate::settings::{ExtraType, Settings};

#[component]
pub fn ExtraTab(
    children: ChildrenFragment,
    names: Vec<String>,
    ids: Vec<ExtraType>,
) -> impl IntoView {
    let setter = use_context::<WriteSignal<Settings>>().expect("to find setter");

    let (get, set) = signal(0);

    names
        .iter()
        .zip(children().nodes.into_iter())
        .zip(ids)
        .enumerate()
        .map(|(n, ((name, child), id))| {
            view! {
                <div class="card block">
                    <header class="card-header is-clickable" on:click=move |_| set(n)>
                        <p class="card-header-title">{name.clone()}</p>
                        <div hidden=move || get() != n>
                            <div class="card-header-icon">
                                <input
                                    class="button is-info is-light is-small ml-2"
                                    type="button"
                                    value="Clear"
                                    on:click=move |_| setter.update(|s| s.clear_extra(id))
                                />
                            </div>
                        </div>
                    </header>

                    <div class="card-content" hidden=move || get() != n >
                        {child}
                    </div>
                </div>
            }
        })
        .collect_view()
}
