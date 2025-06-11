use gloo_net::http::Request;
use yew::prelude::*;
use yew::{function_component, html, Html, Properties};

use crate::pages::vid::Video;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub vid: String,
}

#[function_component(Player)]
pub fn player(props: &Props) -> Html {
    let url = "/matadata/".to_owned() + &props.vid.clone();
    let video = use_state(|| Video {
        id: "".to_string(),
        filename: "".to_string(),
        title: "".to_string(),
        url: "".to_string(),
        date: "".to_string(),
        plot: "".to_string(),
        img: "".to_string(),
    });

    let ved = video.clone();
    use_effect_with(ved.clone(),
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_video: Video = Request::get(&url)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                ved.set(fetched_video);
            });
        }
    );


    return html! {
        <div style="
        padding-top:80px;
        display: 
        flex;
        justify-content:center;
        ">
        <div style="
        display: block;
        ">
            <div>
                <h1>{(*video).title.clone()}</h1>
            </div>
            <video style="max-height: 80vh; max-width: 80vw" id = {(*video).filename.clone()} controls=true src={if (*video).filename.clone() != "".to_string() {format!("/vid/{}",(*video).filename.clone())}else{"".to_string()}} type="video/mp4"/>
            <div>
                <ul style="list-style-type: none;">
                    <li>
                        <google-cast-launcher></google-cast-launcher>
                        {(*video).date.clone()}
                    </li>
                </ul>
                </div>
            </div>
            <script src="https://www.gstatic.com/cv/js/sender/v1/cast_sender.js?loadCastFramework=1"></script>
            <script src="/data/js/ch.js"></script>
        </div>
    };
}
