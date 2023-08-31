use leptos::*;
use crate::AppState;
use crate::Contents;
use crate::update_app_state;

#[component]
pub fn ImageLink(
    cx: Scope,
    contents: Contents,
    app_state_signal: RwSignal<Option<AppState>>,
) -> impl IntoView
{
    let mut mut_key_to = String::from(&contents.key);
    let name_only = mut_key_to.split_off(7);
    let mut date_display = String::from(&contents.last_modified);
    date_display.truncate(10);
    let class_name = "link";
    let comparison_key = String::from(&contents.key);

    view! { cx,
        <div 
            class={class_name}
            id={format!("{}", String::from(&contents.key))}
                on:click={move |_evt| {
                        update_app_state(cx, app_state_signal, AppState::new(contents.clone()));
                    }
                }
            >
                <div class={
                    move ||
                        if app_state_signal.get().expect("app state should exist").current_image == comparison_key 
                        {"link selected"}
                            else 
                        {"link"}
                }>
                    <div>{format!("{}", String::from(&name_only))}</div>
                    <div class="imageDate">{format!("{}", String::from(&date_display))}</div>
                </div>
            </div>
    }
}