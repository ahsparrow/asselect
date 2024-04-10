use leptos::*;

use crate::settings::{AirType, Settings};

#[component]
pub fn AirspaceTab() -> impl IntoView {
    let setter = use_context::<WriteSignal<Settings>>().expect("to find setter");
    let getter = use_context::<ReadSignal<Settings>>().expect("to find getter");

    view! {
        <div class="box">
          <div class="columns">
            <div class="column is-one-third">
              <div class="field">
                <label class="label">
                  "ATZ"
                  <div class="control">
                    <div class="select is-fullwidth">
                      <select on:change=move |ev| { setter.update(|s| s.update("atz", &event_target_value(&ev))) }>
                        <option value=AirType::ClassD.to_string() selected=move || getter().atz == AirType::ClassD>"Class D"</option>
                        <option value=AirType::Ctr.to_string() selected=move || getter().atz == AirType::Ctr>"Control Zone"</option>
                      </select>
                    </div>
                  </div>
                </label>
              </div>
            </div>

            <div class="column is-one-third">
              <div class="field">
                <label class="label">
                  "ILS Feather"
                  <div class="control">
                    <div class="select is-fullwidth">
                      <select on:change=move |ev| { setter.update(|s| s.update("ils", &event_target_value(&ev))) }>
                        <option value="None" selected=move || getter().ils.is_none()>"As ATZ"</option>
                        <option value=AirType::ClassF.to_string() selected=move || getter().ils == Some(AirType::ClassF)>"Class F"</option>
                        <option value=AirType::ClassG.to_string() selected=move || getter().ils == Some(AirType::ClassG)>"Class G"</option>
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
