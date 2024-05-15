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

use crate::settings::{AirType, Settings};

#[component]
pub fn AirspaceTab(gliding_sites: Vec<String>) -> impl IntoView {
    let setter = use_context::<WriteSignal<Settings>>().expect("to find setter");
    let getter = use_context::<ReadSignal<Settings>>().expect("to find getter");

    view! {
        <div class="box">
            <div class="columns">
                <div class="column is-one-third">
                    <div class="field">
                        <label class="label">
                            "ATZ" <div class="control">
                                <div class="select is-fullwidth">
                                    <select on:change=move |ev| {
                                        setter.update(|s| s.update("atz", &event_target_value(&ev)))
                                    }>
                                        <option
                                            value=AirType::ClassD.to_string()
                                            selected=move || getter().atz == AirType::ClassD
                                        >
                                            "Class D"
                                        </option>
                                        <option
                                            value=AirType::Ctr.to_string()
                                            selected=move || getter().atz == AirType::Ctr
                                        >
                                            "Control Zone"
                                        </option>
                                    </select>
                                </div>
                            </div>
                        </label>
                    </div>
                </div>

                <div class="column is-one-third">
                    <div class="field">
                        <label class="label">
                            "ILS Feather" <div class="control">
                                <div class="select is-fullwidth">
                                    <select on:change=move |ev| {
                                        setter.update(|s| s.update("ils", &event_target_value(&ev)))
                                    }>
                                        <option
                                            value="None"
                                            selected=move || getter().ils.is_none()
                                        >
                                            "As ATZ"
                                        </option>
                                        <option
                                            value=AirType::ClassF.to_string()
                                            selected=move || getter().ils == Some(AirType::ClassF)
                                        >
                                            "Class F"
                                        </option>
                                        <option
                                            value=AirType::ClassG.to_string()
                                            selected=move || getter().ils == Some(AirType::ClassG)
                                        >
                                            "Class G"
                                        </option>
                                    </select>
                                </div>
                            </div>
                        </label>
                    </div>
                </div>
            </div>

            <div class="columns">
                <div class="column is-one-third">
                    <div class="field">
                        <label class="label">
                            "Non-ATZ Airfield" <div class="control">
                                <div class="select is-fullwidth">
                                    <select on:change=move |ev| {
                                        setter
                                            .update(|s| {
                                                s.update("unlicensed", &event_target_value(&ev))
                                            })
                                    }>
                                        <option
                                            value="None"
                                            selected=move || getter().unlicensed.is_none()
                                        >
                                            "No"
                                        </option>
                                        <option
                                            value=AirType::ClassF.to_string()
                                            selected=move || {
                                                getter().unlicensed == Some(AirType::ClassF)
                                            }
                                        >
                                            "Class F"
                                        </option>
                                        <option
                                            value=AirType::ClassG.to_string()
                                            selected=move || {
                                                getter().unlicensed == Some(AirType::ClassG)
                                            }
                                        >
                                            "Class G"
                                        </option>
                                    </select>
                                </div>
                            </div>
                        </label>
                    </div>
                </div>

                <div class="column is-one-third">
                    <div class="field">
                        <label class="label">
                            "Microlight Airfield" <div class="control">
                                <div class="select is-fullwidth">
                                    <select on:change=move |ev| {
                                        setter
                                            .update(|s| {
                                                s.update("microlight", &event_target_value(&ev))
                                            })
                                    }>
                                        <option
                                            value="None"
                                            selected=move || getter().microlight.is_none()
                                        >
                                            "No"
                                        </option>
                                        <option
                                            value=AirType::ClassF.to_string()
                                            selected=move || {
                                                getter().microlight == Some(AirType::ClassF)
                                            }
                                        >
                                            "Class F"
                                        </option>
                                        <option
                                            value=AirType::ClassG.to_string()
                                            selected=move || {
                                                getter().microlight == Some(AirType::ClassG)
                                            }
                                        >
                                            "Class G"
                                        </option>
                                    </select>
                                </div>
                            </div>
                        </label>
                    </div>
                </div>
            </div>

            <div class="columns">
                <div class="column is-one-third">
                    <div class="field">
                        <label class="label">
                            "Gliding Airfield"
                            <div class="control">
                                <div class="select is-fullwidth">
                                    <select on:change=move |ev| {
                                        setter
                                            .update(|s| s.update("gliding", &event_target_value(&ev)))
                                    }>
                                        <option
                                            value="None"
                                            selected=move || getter().gliding.is_none()
                                        >
                                            "No"
                                        </option>
                                        <option
                                            value=AirType::Gliding.to_string()
                                            selected=move || {
                                                getter().gliding == Some(AirType::Gliding)
                                            }
                                        >
                                            "Gliding Sector"
                                        </option>
                                        <option
                                            value=AirType::ClassF.to_string()
                                            selected=move || {
                                                getter().gliding == Some(AirType::ClassF)
                                            }
                                        >
                                            "Class F"
                                        </option>
                                        <option
                                            value=AirType::ClassG.to_string()
                                            selected=move || {
                                                getter().gliding == Some(AirType::ClassG)
                                            }
                                        >
                                            "Class G"
                                        </option>
                                    </select>
                                </div>
                            </div>
                        </label>
                    </div>
                </div>

                <div class="column is-one-third">
                    <div class="field">
                        <label class="label">
                            "Exclude Home Airfield" <div class="control">
                                <div class="select is-fullwidth">
                                    <select
                                        name="home"
                                        on:change=move |ev| {
                                            setter
                                                .update(|s| s.update("home", &event_target_value(&ev)))
                                        }
                                    >
                                        <option value="no" selected=move || getter().home.is_none()>"No"</option>
                                        {gliding_sites
                                            .into_iter()
                                            .map(|n|  {
                                                let nc = n.clone();
                                                view! {
                                                    <option selected=move || Some(&n) == getter().home.as_ref()>
                                                        {nc}
                                                    </option>
                                                }
                                            })
                                            .collect_view()}
                                    </select>
                                </div>
                            </div>
                        </label>
                    </div>
                </div>
            </div>
        </div>
    }
}
