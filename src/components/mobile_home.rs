use leptos::*;
use crate::ImageDisplay;
use crate::ImageSelect;
use crate::Banner;
use crate::AppState;
use crate::Contents;

#[component]
pub fn MobileHome(
        cx: Scope,
        image_list: Signal<Option<Vec<Contents>>>,
        app_state_signal: RwSignal<Option<AppState>>,
    ) -> impl IntoView {
    view!{cx, 
        <Banner />
        <div style="display: flex; justify-content: flex-start; flex-direction: column; align-items: flex-start;">
            <ImageSelect image_list = {image_list} app_state_signal={app_state_signal} />    
        </div>
        <ImageDisplay app_state_signal={app_state_signal} image_list={image_list} />
    }.into_view(cx)
}