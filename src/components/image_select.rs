use leptos::*;
use crate::Contents;
use crate::update_app_state;
use crate::AppState;

#[component]
pub fn ImageSelect(
    cx: Scope,
    image_list: Signal<Option<Vec<Contents>>>,
    app_state_signal: RwSignal<Option<AppState>>,
) -> impl IntoView {

    view!{cx, 
        {move || match image_list.get() {
            Some(list) => view!{cx,
                <div style="display:flex; justify-content: center; margin-top: 10px">
                <select
                    style="border: 1px solid black; font-size: 24px; margin: 20px; padding: 20px; width: 600px;"
                    on:change={move |evt| {
                        let image_key = event_target_value(&evt);
                        match image_list.get_untracked() {
                            Some(working_list) => {
                                let mut index = 0;
                                loop {
                                    let item = working_list[index].clone();
                                    if item.key == image_key {
                                        update_app_state(cx, app_state_signal, AppState::new(item));       
                                        break;
                                    }
                                    index = index + 1;
                                    if index > (working_list.len() -1) {break};
                                }
                            },
                            None => (),
                        }
                    }}
                

                >
                    <For
                        each=move || list.to_vec()
                        key=|contents| String::from(&contents.key)
                        view=move |cx, contents: Contents| {
                            let mut mut_key = String::from(&contents.key);
                            let name_only = mut_key.split_off(7);
                            view! {
                                cx,
                                <option 
                                    value={&contents.key}
                                    selected = {move || match app_state_signal.get() {
                                        Some(a) => {
                                            a.current_image == String::from(&contents.key)
                                        },
                                        None => false,
                                    }}
                                >
                                    {name_only}
                                </option>
                            }.into_view(cx)
                        }
                    />
                </select>
                </div>
            }.into_view(cx),
            None => view!{cx, <div>{"loading"}</div>}.into_view(cx),
        }}
    }.into_view(cx)
}