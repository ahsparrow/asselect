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
                            "Gliding Airfield" <div class="control">
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
                                                getter().unlicensed == Some(AirType::Gliding)
                                            }
                                        >
                                            "Gliding Sector"
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
                            "Exclude Home Airfield" <div class="control">
                                <div class="select is-fullwidth">
                                    <select
                                        name="home"
                                        on:change=move |ev| {
                                            setter
                                                .update(|s| s.update("home", &event_target_value(&ev)))
                                        }
                                    >
                                        <option value="no">"No"</option>
                                        {gliding_sites
                                            .into_iter()
                                            .map(|n| view! { <option>{n}</option> })
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
