use leptos::*;

#[component]
pub fn Tabs(tab_names: Vec<String>, children: Children) -> impl IntoView {
    let (selected, set_selected) = create_signal(0);

    view! {
        <nav class="tabs">
            <ul>
                {tab_names
                    .into_iter()
                    .enumerate()
                    .map(|(index, tab_name)| view! {
                        <li class:is-active=move || selected()==index>
                            <a on:click=move |_| set_selected(index)>{tab_name}</a>
                        </li>})
                    .collect_view()
                }
            </ul>
        </nav>

        {children()
            .nodes
            .into_iter()
            .enumerate()
            .map(|(index, child)| view! {
                <div hidden={move || index != selected()}>
                    {child}
                </div>
            })
            .collect_view()
        }
    }
}
