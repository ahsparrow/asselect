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
use leptos::html::div;
use leptos::prelude::*;

use crate::components::select_field::select_field;
use crate::settings::{AirType, Settings};

#[component]
pub fn AirspaceTab(gliding_sites: Vec<String>) -> impl IntoView {
    let setter = use_context::<WriteSignal<Settings>>().expect("to find setter");
    let getter = use_context::<ReadSignal<Settings>>().expect("to find getter");

    let gsites: Vec<&str> = vec!["No"]
        .into_iter()
        .chain(gliding_sites.iter().map(AsRef::as_ref))
        .collect();

    div().child(div().class("box").child((
        div().class("columns").child((
            div().class("column is-one-third").child(select_field(
                setter,
                Signal::derive(move || getter().atz.to_string()),
                "ATZ",
                "atz",
                &vec!["Class D", "Control Zone"],
                &vec![AirType::ClassD.as_ref(), AirType::Ctr.as_ref()],
            )),
            div().class("column is-one-third").child(select_field(
                setter,
                Signal::derive(move || getter().ils.map_or("no".to_string(), |v| v.to_string())),
                "ILS Feather",
                "ils",
                &vec!["As ATZ", "Class F", "Class G"],
                &vec!["no", AirType::ClassF.as_ref(), AirType::ClassG.as_ref()],
            )),
        )),
        div().class("columns").child((
            div().class("column is-one-third").child(select_field(
                setter,
                Signal::derive(move || {
                    getter()
                        .unlicensed
                        .map_or("no".to_string(), |v| v.to_string())
                }),
                "Non-ATZ Airfield",
                "unlicensed",
                &vec!["No", "Class F", "Class G"],
                &vec!["no", AirType::ClassF.as_ref(), AirType::ClassG.as_ref()],
            )),
            div().class("column is-one-third").child(select_field(
                setter,
                Signal::derive(move || {
                    getter()
                        .microlight
                        .map_or("no".to_string(), |v| v.to_string())
                }),
                "Microlight Airfield",
                "microlight",
                &vec!["No", "Class F", "Class G"],
                &vec!["no", AirType::ClassF.as_ref(), AirType::ClassG.as_ref()],
            )),
        )),
        div().class("columns").child((
            div().class("column is-one-third").child(select_field(
                setter,
                Signal::derive(move || {
                    getter().gliding.map_or("no".to_string(), |v| v.to_string())
                }),
                "Gliding Airfield",
                "gliding",
                &vec!["No", "Gliding Sector", "Class F", "Class G"],
                &vec![
                    "no",
                    AirType::Gliding.as_ref(),
                    AirType::ClassF.as_ref(),
                    AirType::ClassG.as_ref(),
                ],
            )),
            div().class("column is-one-third").child(select_field(
                setter,
                Signal::derive(move || getter().home.map_or("No".to_string(), |v| v.to_string())),
                "Exclude Home Airfield",
                "home",
                &gsites,
                &gsites,
            )),
        )),
    )))
}
