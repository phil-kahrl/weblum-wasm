use base64::*;
use base64::alphabet::Alphabet;
use base64::alphabet::ParseAlphabetError;

use crate::AppState;

const CUSTOM_ALPHABET: Result<Alphabet, ParseAlphabetError> = 
    base64::alphabet::Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");

pub struct HashState {
    pub current_image: Option<String>,
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

pub fn decode_hash_route(hash: String) -> HashState {
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

pub fn decode_hash_route_from_url(url: String) -> HashState {
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

pub fn update_hash_route(app_state: Option<AppState>) {
    let engine = base64::engine::GeneralPurpose::new(
        &CUSTOM_ALPHABET.unwrap(),
        base64::engine::general_purpose::PAD);
    if app_state.is_some() {
        leptos::window().location().set_hash(&engine.encode(app_state.expect("")
            .current_image.as_bytes())).expect("unable to set hash");
    }
}

