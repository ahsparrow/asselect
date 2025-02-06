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
use leptos::ev;
use leptos::html::{div, label, option, select};
use leptos::prelude::*;

use crate::Settings;

pub fn select_field(
    setter: WriteSignal<Settings>,
    value: Signal<String>,
    label_str: &str,
    setting: &str,
    options: &Vec<&str>,
    values: &Vec<&str>,
) -> impl IntoView {
    let setting_name = setting.to_string();

    div()
        .class("field")
        .child(
            label().class("label").child((
                label_str.to_string(),
                div().class("control").child(
                    div().class("select is-fullwidth").child(
                        select()
                            .prop("value", move || value.get())
                            .on(ev::change, move |ev| {
                                setter.update(|s| s.update(&setting_name, &event_target_value(&ev)))
                            })
                            .child(
                                options
                                    .iter()
                                    .zip(values)
                                    .map(|(o, v)| {
                                        option()
                                            .value(v.to_string())
                                            .child(o.to_string())
                                            .selected(*v == value.get())
                                    })
                                    .collect_view(),
                            ),
                    ),
                ),
            )),
        )
        .into_any()
}
