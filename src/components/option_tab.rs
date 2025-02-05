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
use crate::settings::{AirType, Format, Overlay, Settings};

pub fn option_tab() -> impl IntoView {
    let setter = use_context::<WriteSignal<Settings>>().expect("to find setter");
    let getter = use_context::<ReadSignal<Settings>>().expect("to find getter");

    div().child(div().class("box").child((
        div().class("columns").child((
            div().class("column is-one-third").child(select_field(
                setter,
                Signal::derive(move || getter.get().format.to_string()),
                "Format",
                "format",
                &vec!["OpenAir", "RA(T) Only", "Competition"],
                &vec![
                    Format::OpenAir.as_ref(),
                    Format::RatOnly.as_ref(),
                    Format::Competition.as_ref(),
                ],
            )),
            div().class("column is-one-third").child(select_field(
                setter,
                Signal::derive(move || getter.get().max_level.to_string()),
                "Maximum Level",
                "max_level",
                &vec!["Unlimited", "FL195", "FL125", "FL105", "FL65"],
                &vec!["660", "195", "125", "105", "65"],
            )),
        )),
        div().class("columns").child((
            div().class("column is-one-third").child(select_field(
                setter,
                Signal::derive(move || {
                    getter
                        .get()
                        .hirta_gvs
                        .map_or("no".to_string(), |v| v.to_string())
                }),
                "HIRTA/GVS",
                "hirta_gvs",
                &vec!["No", "Danger", "Restricted"],
                &vec!["no", AirType::Danger.as_ref(), AirType::Restricted.as_ref()],
            )),
            div().class("column is-one-third").child(select_field(
                setter,
                Signal::derive(move || {
                    getter
                        .get()
                        .obstacle
                        .map_or("no".to_string(), |v| v.to_string())
                }),
                "Obstacle",
                "obstacle",
                &vec!["No", "Danger", "Class F", "Class G"],
                &vec![
                    "no",
                    AirType::Danger.as_ref(),
                    AirType::ClassF.as_ref(),
                    AirType::ClassG.as_ref(),
                ],
            )),
        )),
        div().class("columns").child((
            div().class("column is-one-third").child(select_field(
                setter,
                Signal::derive(move || {
                    if getter.get().radio {
                        "yes".to_string()
                    } else {
                        "no".to_string()
                    }
                }),
                "Radio Frequency",
                "radio",
                &vec!["No", "Add to name"],
                &vec!["no", "yes"],
            )),
            div().class("column is-one-third").child(select_field(
                setter,
                Signal::derive(move || {
                    getter
                        .get()
                        .overlay
                        .map_or("no".to_string(), |v| v.to_string())
                }),
                "Altitude Overlay",
                "overlay",
                &vec![
                    "No",
                    "Bases to FL195",
                    "Bases to FL105",
                    "Bases to FL105 and ATZ/DZ",
                ],
                &vec![
                    "no",
                    Overlay::FL195.as_ref(),
                    Overlay::FL105.as_ref(),
                    Overlay::AtzDz.as_ref(),
                ],
            )),
        )),
    )))
}
