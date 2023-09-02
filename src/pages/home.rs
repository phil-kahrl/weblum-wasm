use leptos::*;
use web_sys::*;
use crate::Contents;
use crate::AppState;
use crate::ImageDisplay;
use crate::ImageList;
use crate::Banner;

#[component]
pub fn Home(cx: Scope, 
  image_list: Signal<Option<Vec<Contents>>>,
  app_state_signal: RwSignal<Option<AppState>>,
  ) 
  -> impl IntoView {
    let (_current_image_key, _set_current_image_key) = create_signal(cx, "");
    let file_option: Option<File> = None;
    let (_file, _set_file) = create_signal(cx, file_option);
    view! { cx,
        <Banner />
        <div style = {"display: flex; flex-direction: row"} >
            <ImageList image_list = {image_list} app_state_signal = {app_state_signal} />
            <ImageDisplay app_state_signal={app_state_signal} image_list={image_list} />
        </div>
    }
}
