use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use crate::api::S3Api;

use crate::hash_route::*;

mod api;
mod hash_route;
mod components;
mod pages;

use self::{components::*, pages::*};

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub message: String,
}

#[derive(Clone, Deserialize)]
pub struct AppState {
    current_image: String,
    e_tag: String,
    current_caption: Option<String>,
}

impl AppState {
    pub fn new(contents: Contents) -> Self {
        Self {
            current_image: contents.key,
            e_tag: contents.e_tag,
            current_caption: None,  
        }
    }
    pub fn set_caption(&mut self, new_caption: String) {
        self.current_caption = Some(new_caption);
    }
}

#[derive(Clone, Deserialize)]
pub struct ListBucketResult {
    #[serde(rename(deserialize = "NextContinuationToken"))]
    next_continuation_token: Option<String>,
    #[serde(rename(deserialize = "IsTruncated"))]
    is_truncated: bool,
    #[serde(rename(deserialize = "Contents"))]
    contents: Vec<Contents>,
}

impl ListBucketResult {
    pub fn new() -> Self {
        Self { 
            next_continuation_token: None,
            is_truncated: false,
            contents: vec![],
        }
    }
}

#[derive(Clone, Deserialize, Ord, Eq, PartialOrd, PartialEq)]
pub struct Contents {
    #[serde(rename(deserialize = "Key"))]
    key: String,
    #[serde(rename(deserialize = "LastModified"))]
    last_modified: String,
    #[serde(rename(deserialize = "ETag"))]
    e_tag: String,
    #[serde(rename(deserialize = "Size"))]
    _size: i64,
    #[serde(rename(deserialize = "StorageClass"))]
    _storage_class: String,
}

fn get_update_caption_action(cx: Scope, app_state: RwSignal<Option<AppState>>) -> Action<(), ()> {
    create_action(cx, move |_| async move {
        let s3_api = S3Api::new();
        let ut = app_state.get_untracked().expect("app state expected");
        match s3_api.get_comment(ut.e_tag).await {
            Some(contents) => {
                let mut current_state= app_state.get_untracked().expect("app state expected");
                current_state.set_caption(contents);
                app_state.set(Some(current_state));
            },
            _ => log::info!("No contents returned"),
        }
    })
}

pub fn update_app_state(cx: Scope, app_state_signal: RwSignal<Option<AppState>>, new_app_state: AppState) {
    app_state_signal.set(Some(new_app_state));
    get_update_caption_action(cx, app_state_signal).dispatch(());
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let image_list = create_rw_signal(cx, None::<Vec<Contents>>);
    let app_state_signal = create_rw_signal(cx, None::<AppState>);

    window_event_listener(ev::hashchange, move |ev| {
        let hash_state = decode_hash_route_from_url(ev.new_url());

        if hash_state.current_image.is_some() {
            let image_key = hash_state.current_image.expect("No image key found");
            let working_list = image_list.get_untracked().expect("image list empty");
            let mut index = 0;
            loop {
                let item = working_list[index].clone();
                if item.key == image_key {
                    log::info!("setting key from hash");
                    update_app_state(cx, app_state_signal, AppState::new(item));       
                    break;
                }
                index = index + 1;
                if index > (working_list.len() -1) {break};
            }
        }
    });

    let fetch_images = create_action(cx, move |_| async move {
        let s3_api = api::S3Api::new();
        match s3_api.list_images().await {
            Ok(contents) => {
                let mut found_current_in_hash_route = false;
                if leptos::window().location().hash().is_ok() {
                    let mut hash = leptos::window().location().hash().expect("no hash found");
                    if hash.len() > 0 {
                        hash.remove(0);
                        let current_hash_state = decode_hash_route(hash);

                        if current_hash_state.current_image.is_some() {
                            let image_key = current_hash_state.current_image.expect("no image found");
                            let mut index = 0;
                            loop {
                                let item = contents[index].clone();
                                if item.key == image_key {
                                    log::info!("setting key from hash");
                                    let new_state = AppState::new(item);
                                    update_app_state(cx, app_state_signal, new_state);
                                    found_current_in_hash_route = true;
                                    break;
                                }
                                index = index + 1;
                                if index > (contents.len() -1) {break};
                            }
                        }
                    }
                }
                if !found_current_in_hash_route {
                    let ci = contents.first().expect("image list empty");
                    update_app_state(cx, app_state_signal, AppState::new(ci.clone()).clone());  
                }
                image_list.set(Some(contents)); 
            }
            Err(err) => {
                log::info!("error");
                log::error!("error: {err}");
                image_list.set(Some(vec![]));
            }
        }
    });

    create_effect(cx, move |_| {
        match  app_state_signal.get() {
            Some(a) => update_hash_route(Some(a)),
            _ => log::info!("No state to update")
        }
    });
        

    // -- initial image fetch //
    log::info!("initializing");
    fetch_images.dispatch(());

    view! { cx,
      <Router>
        <main>
          <Routes>
            <Route
              path=Page::Home.path()
              view=move |cx| view! { cx,
                <Home image_list = image_list.into() app_state_signal = app_state_signal.into() />
              }
            />
          </Routes>
        </main>
      </Router>
    }
}
