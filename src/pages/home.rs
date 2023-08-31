use leptos::*;
use web_sys::*;
use crate::Contents;
use crate::AppState;
use crate::ImageLink;
use crate::ImageDisplay;

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
      <h2>"Weblum Photos"</h2>
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
            <div>
              <ImageDisplay app_state_signal={app_state_signal} image_list={image_list} />
            </div>
          </div>
        }.into_view(cx),
        None => view!{ cx,
          <p>"Loading ..."</p>
        }.into_view(cx)
      }}
    }
}
