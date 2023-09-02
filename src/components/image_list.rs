use leptos::*;
use crate::ImageLink;
use crate::Contents;
use crate::AppState;

#[component]
pub fn ImageList(
    cx: Scope,
    image_list: Signal<Option<Vec<Contents>>>,
    app_state_signal: RwSignal<Option<AppState>>,
) -> impl IntoView {

    {move || match image_list.get() {
        Some(list) => view!{ cx,
          <div class="listContainer">
            <div class="imageList">
              <For
                each=move || list.to_vec()
                key=|contents| String::from(&contents.key)
                view=move |cx, contents: Contents| {
                  view! {
                    cx,
                    <ImageLink 
                      contents={contents}
                      app_state_signal={app_state_signal.into()}
                    />
                  }
                }
              />
            </div>
          </div>
        }.into_view(cx),
        None => view!{ cx,
          <p>"Loading ..."</p>
        }.into_view(cx)
    }}
}