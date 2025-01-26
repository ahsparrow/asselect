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

#[component]
pub fn Tabs(tab_names: Vec<String>, children: ChildrenFragment) -> impl IntoView {
    let (selected, set_selected) = signal(0);

    view! {
        <nav class="tabs">
            <ul>
                {tab_names
                    .into_iter()
                    .enumerate()
                    .map(|(index, tab_name)| {
                        view! {
                            <li class:is-active=move || selected() == index>
                                <a on:click=move |_| set_selected(index)>{tab_name}</a>
                            </li>
                        }
                    })
                    .collect_view()}

            </ul>
        </nav>

        <div class="mx-4">
            {children()
                .nodes
                .into_iter()
                .enumerate()
                .map(|(index, child)| view! { <div hidden=move || index != selected()>{child}</div> })
                .collect_view()}
        </div>
    }
}
