use leptos::*;

use crate::settings::{AirType, Settings};

#[component]
pub fn AirspaceTab(getter: ReadSignal<Settings>, setter: WriteSignal<Settings>) -> impl IntoView {
    view! {
        <div class="box">
          <div class="columns">
            <div class="column is-one-third">
              <div class="field">
                <label class="label">
                  "ATZ"
                  <div class="control">
                    <div class="select is-fullwidth">
                      <select name="atz" on:change=move |ev| { setter.update(|s| s.update("atz", &event_target_value(&ev))) }>
                        <option value=AirType::ClassD.to_string() selected=getter().atz == AirType::ClassD>"Class D"</option>
                        <option value=AirType::Ctr.to_string() selected=getter().atz == AirType::Ctr>"Control Zone"</option>
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
                      <select name="ils" >
                        <option value="None" selected=getter().ils == None>"As ATZ"</option>
                        <option value=AirType::ClassF.to_string() selected=getter().ils == Some(AirType::ClassF)>"Class F"</option>
                        <option value=AirType::ClassG.to_string() selected=getter().ils == Some(AirType::ClassG)>"Class G"</option>
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
