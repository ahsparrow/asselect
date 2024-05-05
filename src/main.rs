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
use futures::join;
use gloo::file::{Blob, ObjectUrl};
use gloo::net::http::Request;
use leptos::*;
use leptos_use::storage::use_local_storage;
use leptos_use::utils::JsonCodec;

use components::{
    about_tab::AboutTab, airspace_tab::AirspaceTab, extra_panel::ExtraPanel, extra_tab::ExtraTab,
    notam_tab::NotamTab, option_tab::OptionTab, tabs::Tabs,
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

#[component]
fn App() -> impl IntoView {
    let async_yaixm = create_local_resource(|| (), |_| async move { fetch_yaixm().await });

    let async_overlay = create_local_resource(
        || (),
        |_| async move {
            let overlay_195 = fetch_overlay("overlay_195.txt");
            let overlay_105 = fetch_overlay("overlay_105.txt");
            let overlay_atzdz = fetch_overlay("overlay_atzdz.txt");
            let (o_195, o_105, o_atzdz) = join!(overlay_195, overlay_105, overlay_atzdz);
            OverlayData {
                overlay_195: o_195,
                overlay_105: o_105,
                overlay_atzdz: o_atzdz,
            }
        },
    );

    view! {
        {move || match async_yaixm.get() {
            Some(resource) => {
                match resource {
                    Some(yaixm) => view! { <MainView yaixm=yaixm overlay=async_overlay/> }.into_view(),
                    None => view! { <p>"Error getting airspace data"</p> }.into_view(),
                }
            }
            None => view! { <p>"Getting airspace data, please wait..."</p> }.into_view(),
        }}
    }
}

#[component]
fn MainView(yaixm: Yaixm, overlay: Resource<(), OverlayData>) -> impl IntoView {
    // Local settings storage
    let (local_settings, set_local_settings, _) =
        use_local_storage::<Settings, JsonCodec>("settings");

    // Make copy of settings so store value is only updated on download
    let (settings, set_settings) = create_signal(local_settings.get_untracked());
    provide_context(settings);
    provide_context(set_settings);

    // Release note modal display control
    let (modal, set_modal) = create_signal(false);

    // UI data from YAIXM
    let rat_names = rat_names(&yaixm);
    let mut loa_names = loa_names(&yaixm);
    let mut wave_names = wave_names(&yaixm);
    loa_names.sort();
    wave_names.sort();

    let mut gliding_sites = gliding_sites(&yaixm);
    gliding_sites.sort();

    let airac_date = yaixm.release.airac_date[..10].to_string();
    let release_note = yaixm.release.note.clone();

    // UI static data
    let tab_names = vec![
        "Main".to_string(),
        "Option".to_string(),
        "Extra".to_string(),
        "NOTAM".to_string(),
        "About".to_string(),
    ];

    let extra_names = vec![
        "Temporary Restrictions".to_string(),
        "Local Agreements".to_string(),
        "Wave Boxes".to_string(),
    ];

    let extra_ids = vec![ExtraType::Rat, ExtraType::Loa, ExtraType::Wave];

    // Download button callback
    let download = move |_| {
        // Store settings
        set_local_settings.set(settings.get_untracked());

        let user_agent = web_sys::window()
            .and_then(|w| w.navigator().user_agent().ok())
            .unwrap_or_default();

        // Create OpenAir data
        let oa = openair(&yaixm, &settings.get_untracked(), &user_agent);

        // Get overlay data
        let od = if let Some(overlay_setting) = settings().overlay {
            if let Some(overlay_data) = overlay.get() {
                let x = match overlay_setting {
                    Overlay::FL195 => overlay_data.overlay_195,
                    Overlay::FL105 => overlay_data.overlay_105,
                    Overlay::AtzDz => overlay_data.overlay_atzdz,
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

        // Trigger a "fake" download
        let a = leptos::html::a();
        a.set_download("openair.txt");
        a.set_href(&object_url);
        a.click();
    };

    view! {
        <header class="hero is-small is-primary block">
            <div class="hero-body">
                <div class="container">
                    <div class="title is-4">{"ASSelect - UK Airspace"}</div>
                </div>
            </div>
        </header>

        <div class="container block">
            <Tabs tab_names>
                <AirspaceTab gliding_sites=gliding_sites/>
                <OptionTab/>
                <ExtraTab names=extra_names ids=extra_ids>
                    <ExtraPanel names=rat_names id=ExtraType::Rat/>
                    <ExtraPanel names=loa_names id=ExtraType::Loa/>
                    <ExtraPanel names=wave_names id=ExtraType::Wave/>
                </ExtraTab>
                <NotamTab/>
                <AboutTab/>
            </Tabs>
        </div>

        <div class="container block">
            <div class="mx-4">
                <button type="submit" class="button is-primary" on:click=download>
                    {"Get Airspace"}
                </button>

                <a id="airac-button" class="button is-text is-pulled-right" on:click=move |_| set_modal(true)>
                    "AIRAC: "{ airac_date }
                </a>
            </div>
        </div>

        // Release note overlay
        <div class="modal" class:is-active=modal>
            <div class="modal-background"></div>
                <div class="modal-content">
                    <div class="box">
                        <h2 class="subtitle">{"Release Details"}</h2>
                        <pre>{ release_note }</pre>
                    </div>
                </div>
            <button class="modal-close is-large" on:click=move |_| set_modal(false)></button>
        </div>
    }
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
    mount_to_body(|| view! { <App/> })
}
