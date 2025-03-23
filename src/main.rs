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
use codee::string::JsonSerdeCodec;
use futures::join;
use gloo::file::{Blob, ObjectUrl};
use gloo::net::http::Request;
use leptos::ev;
use leptos::html::{a, button, div, h2, header, p, pre, A};
use leptos::prelude::*;
use leptos::web_sys;
use leptos_use::storage::use_local_storage;

use components::{
    about_tab::about_tab, airspace_tab::airspace_tab, extra_panel::extra_panel,
    extra_tab::extra_tab, notam_tab::notam_tab, option_tab::option_tab, tabs::tabs,
};
use convert::openair;
use settings::{ExtraType, Overlay, Settings};
use yaixm::{gliding_sites, loa_names, rat_names, wave_names, Yaixm};

mod components;
mod convert;
mod settings;
mod yaixm;

#[derive(Clone, Debug)]
struct OverlayData {
    overlay_195: Option<String>,
    overlay_105: Option<String>,
    overlay_atzdz: Option<String>,
}

fn app() -> impl IntoView {
    let async_yaixm = LocalResource::new(fetch_yaixm);

    let async_overlay = LocalResource::new(|| async {
        let overlay_195 = fetch_overlay("overlay_195.txt");
        let overlay_105 = fetch_overlay("overlay_105.txt");
        let overlay_atzdz = fetch_overlay("overlay_atzdz.txt");
        let (o_195, o_105, o_atzdz) = join!(overlay_195, overlay_105, overlay_atzdz);
        OverlayData {
            overlay_195: o_195,
            overlay_105: o_105,
            overlay_atzdz: o_atzdz,
        }
    });

    move || match async_yaixm.get().as_deref() {
        Some(resource) => match resource {
            Some(yaixm) => {
                // This needs to use view! macro, otherwise reactive system breaks. Don't know why
                view! {<MainView yaixm=yaixm.clone() overlay=async_overlay />}.into_any()
            }
            None => p().child("Error getting airspace data").into_any(),
        },
        None => p()
            .child("Getting airspace data, please wait...")
            .into_any(),
    }
}

#[component]
fn MainView(yaixm: Yaixm, overlay: LocalResource<OverlayData>) -> impl IntoView {
    // Local settings storage
    let (local_settings, set_local_settings, _) =
        use_local_storage::<Settings, JsonSerdeCodec>("settings");

    // Make copy of settings so store value is only updated on download
    let (settings, set_settings) = signal(local_settings.get_untracked());
    provide_context(settings);
    provide_context(set_settings);

    // Release note modal display control
    let (modal, set_modal) = signal(false);

    // UI data from YAIXM
    let rat_names = rat_names(&yaixm);
    let mut loa_names = loa_names(&yaixm);
    let mut wave_names = wave_names(&yaixm);
    loa_names.sort();
    wave_names.sort();

    // Clean old RATs from settings
    for rat_name in settings.get().rat {
        if !rat_names.contains(&rat_name) {
            set_local_settings.update(|s| {
                s.rat.remove(&rat_name);
            });
        }
    }

    let mut gliding_sites = gliding_sites(&yaixm);
    gliding_sites.sort();

    let airac_date = yaixm.release.airac_date[..10].to_string();
    let release_note = yaixm.release.note.clone();
    let filename = format!("uk{}.txt", airac_date);

    // UI static data
    let tab_names = vec!["Main", "Option", "Extra", "NOTAM", "About"];

    let extra_names = vec!["Temporary Restrictions", "Local Agreements", "Wave Boxes"];
    let extra_ids = vec![ExtraType::Rat, ExtraType::Loa, ExtraType::Wave];

    let download_node_ref = NodeRef::<A>::new();

    // Download button callback
    let download = move |_| {
        // Store settings
        set_local_settings.set(settings.get_untracked());

        let user_agent = web_sys::window()
            .and_then(|w| w.navigator().user_agent().ok())
            .unwrap_or_default();

        // Create OpenAir data
        let oa = if settings.get().overlay != Some(Overlay::AtzDzOnly) {
            openair(&yaixm, &settings.get_untracked(), &user_agent)
        } else {
            // Overlay only, no airspace
            "".to_string()
        };

        // Get overlay data
        let od = if let Some(overlay_setting) = settings.get().overlay {
            if let Some(overlay_data) = overlay.get().as_deref() {
                let x = match overlay_setting {
                    Overlay::FL195 => overlay_data.overlay_195.clone(),
                    Overlay::FL105 => overlay_data.overlay_105.clone(),
                    Overlay::AtzDz | Overlay::AtzDzOnly => overlay_data.overlay_atzdz.clone(),
                };
                x.unwrap_or("* Missing overlay data".to_string())
            } else {
                "* Overlay data not loaded".to_string()
            }
        } else {
            "".to_string()
        };

        // Create download data
        let blob = Blob::new((oa + od.as_str()).as_str());
        let object_url = ObjectUrl::from(blob);

        let a = download_node_ref.get().unwrap();
        a.set_download(&filename);
        a.set_href(&object_url);
        a.click();
    };

    let children = vec![
        airspace_tab(gliding_sites).into_any(),
        option_tab().into_any(),
        extra_tab(
            vec![
                extra_panel(rat_names, ExtraType::Rat).into_any(),
                extra_panel(loa_names, ExtraType::Loa).into_any(),
                extra_panel(wave_names, ExtraType::Wave).into_any(),
            ],
            extra_names,
            extra_ids,
        )
        .into_any(),
        notam_tab().into_any(),
        about_tab().into_any(),
    ];

    (
        // Page header
        header()
            .class("hero is-small has-background-primary-soft block")
            .child(
                div().class("hero-body").child(
                    div().class("container").child(
                        div()
                            .class("title is-4 has-text-primary-soft-invert")
                            .child("ASSelect - UK Airspace"),
                    ),
                ),
            ),
        // Tabs
        div()
            .class("container block")
            .child(tabs(tab_names, children)),
        // Buttons
        div().class("container block").child(
            div().class("mx-4").child((
                button()
                    .r#type("submit")
                    .class("button is-primary has-text-primary-100")
                    .on(ev::click, download)
                    .child("Get Airspace"),
                a().id("airac-button")
                    .class("button is-text is-pulled-right")
                    .on(ev::click, move |_| set_modal.set(true))
                    .child(format!("AIRAC: {}", airac_date)),
            )),
        ),
        // Release note overlay
        div()
            .class(move || {
                if modal.get() {
                    "modal is-active"
                } else {
                    "modal"
                }
            })
            .child((
                div().class("modal-background"),
                div()
                    .class("modal-content")
                    .child(div().class("box").child((
                        h2().class("subtitle").child("Release Details"),
                        pre().child(release_note),
                    ))),
                button()
                    .class("modal-close is-large")
                    .on(ev::click, move |_| set_modal.set(false)),
            )),
        // For data download
        a().hidden(true).node_ref(download_node_ref),
    )
}

// Get YAIXM data from server
async fn fetch_yaixm() -> Option<Yaixm> {
    let result = Request::get("yaixm.json").send().await;
    match result {
        Ok(response) => response.json().await.ok(),
        _ => None,
    }
}

// Get overlay data from server
async fn fetch_overlay(path: &str) -> Option<String> {
    let result = Request::get(path).send().await;
    match result {
        Ok(response) => response.text().await.ok(),
        _ => None,
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(app)
}
