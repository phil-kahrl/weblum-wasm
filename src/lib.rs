use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use base64::*;
use base64::alphabet::Alphabet;
use base64::alphabet::ParseAlphabetError;

const CUSTOM_ALPHABET: Result<Alphabet, ParseAlphabetError> = 
    base64::alphabet::Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");


mod api;
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
}

impl AppState {
    pub fn new(ci: String) -> Self {
        Self {
            current_image: ci,
        }
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

struct HashState {
    current_image: Option<String>,
}

impl HashState {
        pub fn new(current_image: Option<String>) -> Self {
            Self { 
                current_image: current_image,
            }
        }

        pub fn empty() -> Self {
            Self {
                current_image: None
            }
        }
}


fn decode_hash_route_from_url(url: String) -> HashState {
    let mut url = String::from(&url);
    let mut hash_state = HashState::empty();
    let hash_index = url.find("#");
    if hash_index.is_some() {
        let mut hash = url.split_off(hash_index.unwrap());
        hash.remove(0);
        hash_state = decode_hash_route(hash);
    }
    hash_state

}

fn decode_hash_route(hash: String) -> HashState {
    let engine = base64::engine::GeneralPurpose::new(
        &CUSTOM_ALPHABET.unwrap(),
        base64::engine::general_purpose::PAD);
    let decoded = engine.decode(&String::from(&hash));
    let mut image_key: Option<String> = None;
    if decoded.is_ok() {
        let ds =  String::from_utf8(decoded.unwrap());
        if ds.is_ok() {
            let s = String::from_utf8(ds.unwrap().into());
            if s.is_ok() {
                image_key = Some(s.clone().unwrap());
            }
        }
    }
    HashState::new(image_key)
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let image_list = create_rw_signal(cx, None::<Vec<Contents>>);
    let app_state = create_rw_signal(cx, None::<AppState>);

    window_event_listener(ev::hashchange, move |ev| {
        let hash_state = decode_hash_route_from_url(ev.new_url());

        if hash_state.current_image.is_some() {
            let image_key = hash_state.current_image.expect("No image key found");
            let working_list = image_list.get_untracked().expect("image list empty");
            let mut index = 0;
            loop {
                let item = &working_list[index];
                if item.key == image_key {
                    log::info!("setting key from hash");
                    app_state.set(Some(AppState::new(String::from(&item.key))));
                    break;
                }
                index = index + 1;
                if index > (working_list.len() -1) {break};
            }
        }
    });

    create_effect(cx, move |_| {
        let engine = base64::engine::GeneralPurpose::new(
            &CUSTOM_ALPHABET.unwrap(),
            base64::engine::general_purpose::PAD);
        let ci = app_state.get();
        if ci.is_some() {
            leptos::window().location().set_hash(&engine.encode(ci.expect("")
                .current_image.as_bytes())).expect("unable to set hash");
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
                                let item = &contents[index];
                                if item.key == image_key {
                                    log::info!("setting key from hash");
                                    app_state.set(Some(AppState::new(String::from(&item.key))));
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
                    let ci = &contents.first().expect("image list empty").key;
                    let new_app_state = AppState::new(ci.to_string());
                    app_state.set(Some(new_app_state));
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
                <Home image_list = image_list.into() app_state = app_state.into() />
              }
            />
          </Routes>
        </main>
      </Router>
    }
}
