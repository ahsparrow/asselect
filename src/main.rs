use gloo::file::{Blob, ObjectUrl};
use gloo::net::http::Request;
use leptos::*;

use components::{
    airspace_tab::AirspaceTab, extra_panel::ExtraPanel, extra_tab::ExtraTab, notam_tab::NotamTab,
    option_tab::OptionTab, tabs::Tabs,
};
use settings::ExtraType;
use yaixm::{gliding_sites, loa_names, rat_names, wave_names, Yaixm};

mod components;
mod settings;
mod yaixm;

#[component]
fn App() -> impl IntoView {
    let async_data = create_local_resource(|| (), |_| async move { fetch_yaixm().await });

    view! {
        { move || match async_data.get() {
            Some(resource) => match resource {
                Some(yaixm) => {
                    view! { <MainView yaixm=yaixm/> }.into_view()
                },
                None => view! { <p>"Error loading YAXIM"</p> }.into_view()
            }
            None => view! { <p>"Loading YAIXM, please wait..."</p> }.into_view()
        }}
    }
}

#[component]
fn MainView(yaixm: Yaixm) -> impl IntoView {
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

    let (settings, set_settings) = create_signal(settings::Settings::default());
    provide_context(settings);
    provide_context(set_settings);

    let mut gliding_sites = gliding_sites(&yaixm);
    gliding_sites.sort();

    view! {
        <header class="hero is-small is-primary block">
            <div class="hero-body">
            <div class="container">
                <div class="title is-4">
                {"ASSelect - UK Airspace"}
                </div>
            </div>
            </div>
        </header>

        <div class="container block">
            <Tabs tab_names>
                <AirspaceTab gliding_sites=gliding_sites />
                <OptionTab />
                <ExtraTab names=extra_names ids=extra_ids>
                    <ExtraPanel names=rat_names(&yaixm) id=ExtraType::Rat />
                    <ExtraPanel names=loa_names(&yaixm) id=ExtraType::Loa />
                    <ExtraPanel names=wave_names(&yaixm) id=ExtraType::Wave />
                </ExtraTab>
                <NotamTab />
            </Tabs>
        </div>

        <div class="container block">
            <div class="mx-4">
            <button type="submit" class="button is-primary"
                on:click = move |_| {
                    logging::log!("{:?}", settings());

                    let blob = Blob::new("Hello Alan");
                    let object_url = ObjectUrl::from(blob);

                    let a = leptos::html::a();
                    a.set_download("openair.txt");
                    a.set_href(&object_url);
                    a.click();
                }>
                {"Get Airspace"}
            </button>
            </div>
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

fn main() {
    mount_to_body(|| view! { <App/> })
}
