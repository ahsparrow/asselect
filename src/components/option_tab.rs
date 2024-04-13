use leptos::*;

use crate::settings::{AirType, Format, Overlay, Settings};

#[component]
pub fn OptionTab() -> impl IntoView {
    let setter = use_context::<WriteSignal<Settings>>().expect("to find setter");
    let getter = use_context::<ReadSignal<Settings>>().expect("to find getter");

    view! {
        <div class="box">
            <div class="columns">
                <div class="column is-one-third">
                    <div class="field">
                        <label class="label">
                            "Format"
                            <div class="control">
                                <div class="select is-fullwidth">
                                    <select on:change=move |ev| {
                                        setter.update(|s| s.update("format", &event_target_value(&ev)))
                                    }>
                                        <option
                                            value=Format::OpenAir.to_string()
                                            selected=move || getter().format == Format::OpenAir
                                        >
                                            "OpenAir"
                                        </option>
                                        <option
                                            value=Format::RatOnly.to_string()
                                            selected=move || getter().format == Format::RatOnly
                                        >
                                            "RA(T) Only"
                                        </option>
                                        <option
                                            value=Format::Competition.to_string()
                                            selected=move || getter().format == Format::Competition
                                        >
                                            "Competition"
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
                            "Maximum Level"
                            <div class="control">
                                <div class="select is-fullwidth">
                                    <select on:change=move |ev| {
                                        setter.update(|s| s.update("max_level", &event_target_value(&ev)))
                                    }>
                                        <option
                                            value="660"
                                            selected=move || getter().max_level == 660
                                        >
                                            "Unlimited"
                                        </option>
                                        <option
                                            value="195"
                                            selected=move || getter().max_level == 195
                                        >
                                            "FL195"
                                        </option>
                                        <option
                                            value="125"
                                            selected=move || getter().max_level == 125
                                        >
                                            "FL125"
                                        </option>
                                        <option
                                            value="105"
                                            selected=move || getter().max_level == 105
                                        >
                                            "FL105"
                                        </option>
                                        <option
                                            value="65"
                                            selected=move || getter().max_level == 65
                                        >
                                            "FL65"
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
                            "HIRTA/GVS"
                            <div class="control">
                                <div class="select is-fullwidth">
                                    <select on:change=move |ev| {
                                        setter
                                            .update(|s| {
                                                s.update("hirta_gvs", &event_target_value(&ev))
                                            })
                                    }>
                                        <option
                                            value="None"
                                            selected=move || getter().hirta_gvs.is_none()
                                        >
                                            "No"
                                        </option>
                                        <option
                                            value=AirType::Danger.to_string()
                                            selected=move || {
                                                getter().hirta_gvs == Some(AirType::Danger)
                                            }
                                        >
                                            "Danger"
                                        </option>
                                        <option
                                            value=AirType::Restricted.to_string()
                                            selected=move || {
                                                getter().hirta_gvs == Some(AirType::Restricted)
                                            }
                                        >
                                            "Restricted"
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
                            "Obstacle"
                            <div class="control">
                                <div class="select is-fullwidth">
                                    <select on:change=move |ev| {
                                        setter
                                            .update(|s| {
                                                s.update("obstacle", &event_target_value(&ev))
                                            })
                                    }>
                                        <option
                                            value="None"
                                            selected=move || getter().obstacle.is_none()
                                        >
                                            "No"
                                        </option>
                                        <option
                                            value=AirType::Danger.to_string()
                                            selected=move || {
                                                getter().obstacle == Some(AirType::Danger)
                                            }
                                        >
                                            "Danger"
                                        </option>
                                        <option
                                            value=AirType::ClassF.to_string()
                                            selected=move || {
                                                getter().obstacle == Some(AirType::ClassF)
                                            }
                                        >
                                            "Class F"
                                        </option>
                                        <option
                                            value=AirType::ClassG.to_string()
                                            selected=move || {
                                                getter().obstacle == Some(AirType::ClassG)
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
                            "Radio Frequency"
                            <div class="control">
                                <div class="select is-fullwidth">
                                    <select on:change=move |ev| {
                                        setter
                                            .update(|s| {
                                                s.update("radio", &event_target_value(&ev))
                                            })
                                    }>
                                        <option
                                            value="no"
                                            selected=move || !getter().radio
                                        >
                                            "No"
                                        </option>
                                        <option
                                            value="yes"
                                            selected=move || getter().radio
                                        >
                                            "Add to name"
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
                            "Altitude Overlay"
                            <div class="control">
                                <div class="select is-fullwidth">
                                    <select on:change=move |ev| {
                                        setter
                                            .update(|s| {
                                                s.update("overlay", &event_target_value(&ev))
                                            })
                                    }>
                                        <option
                                            value="no"
                                            selected=move || getter().overlay.is_none()
                                        >
                                            "None"
                                        </option>
                                        <option
                                            value=Overlay::FL195.to_string()
                                            selected=move || getter().overlay  == Some(Overlay::FL195)
                                        >
                                            "Bases to FL195"
                                        </option>
                                        <option
                                            value=Overlay::FL105.to_string()
                                            selected=move || getter().overlay == Some(Overlay::FL105)
                                        >
                                            "Bases to FL105"
                                        </option>
                                        <option
                                            value=Overlay::AtzDz.to_string()
                                            selected=move || getter().overlay == Some(Overlay::AtzDz)
                                        >
                                            "Bases to FL105 and ATZ/DZ"
                                        </option>
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
