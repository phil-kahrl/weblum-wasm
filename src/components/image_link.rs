use leptos::*;
use crate::AppState;

#[component]
pub fn ImageLink(
    cx: Scope,
    key: String,
    _id: String,
    last_modified: String,
    app_state: RwSignal<Option<AppState>>,
) -> impl IntoView
{
    let click_arg = String::from(&key);
    let mut mut_key_to = String::from(&key);
    let name_only = mut_key_to.split_off(7);
    let mut date_display = String::from(&last_modified);
    date_display.truncate(10);
    let class_name = "link";
    let comparison_key = String::from(&key);

    view! { cx,
        <div 
            class={class_name}
            id={format!("{}", String::from(&key))}
                on:click={move |_evt| {
                        app_state.set(Some(AppState::new(format!("{}", String::from(&click_arg)))));
                    }
                }
            >
                <div class={ 
                    move || 
                        if app_state.get().unwrap_or(AppState::new("none".to_string())).current_image == comparison_key 
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