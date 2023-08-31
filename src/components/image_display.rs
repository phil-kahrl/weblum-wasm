use leptos::*;
use crate::AppState;
use crate::Contents;
use crate::api::S3Api;

#[component]
pub fn ImageDisplay(
    cx: Scope,
    app_state: RwSignal<Option<AppState>>,
    image_list: Signal<Option<Vec<Contents>>>,
) -> impl IntoView
{
    {
        move || match app_state.get() {
            Some(s) => view!{ cx,
                <div class="imageDisplay">
                    <div class="imageButtonsContainer">
                        <div 
                            class="link"
                            on:click={move |_evt| {
                                let mut index = 0;
                                let working_list = image_list.get_untracked().expect("image list empty");
                                let mut next_index = 0;
                                loop {
                                    let item = &working_list[index];
                                    if app_state.get_untracked().expect("current image not found").current_image == item.key {
                                        if index > 0 {next_index = index - 1};
                                        break;
                                    }
                                    index = index + 1;
                                }
                                let next_image = &working_list[next_index];
                                app_state.set(Some(AppState::new(String::from(&next_image.key))));             
                            }}
                            >{"Previous"}       
                        </div>
                        <h3>{format!("{}", s.current_image.replace("images/", ""))}</h3>
                        <div 
                            class="link"
                            on:click={move |_evt| {
                                let mut index = 0;
                                let working_list = image_list.get_untracked().expect("image list empty");
                                let mut next_index = working_list.len() - 1;
                                 loop {
                                    let item = &working_list[index];
                                    if app_state.get_untracked().expect("current image not found").current_image == item.key {
                                        if index < working_list.len() -1 {next_index = index + 1};
                                        break;
                                    }
                                    index = index + 1;
                                }
                                let next_image = &working_list[next_index];
                                app_state.set(Some(AppState::new(String::from(&next_image.key))));             
                            }}
                            >{"Next"}       
                        </div>
                    </div>
                    <img
                        src={format!("http://{}.s3.amazonaws.com/{}", S3Api::new().bucket_name(), s.current_image)}
                    />
                    <a class="link" target={"_blank"} href={format!("http://{}.s3.amazonaws.com/{}", S3Api::new().bucket_name(), s.current_image)}>
                        {"Download Image"}
                    </a>
                </div>
            },
            None => view!{ cx, <div>"no image selected "</div>},
        }
    }
}