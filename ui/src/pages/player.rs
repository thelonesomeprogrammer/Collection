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
    {
        let video = video.clone();
        use_effect_with_deps(
            move |_| {
                let video = video.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_video: Video = Request::get(&url)
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    video.set(fetched_video);
                });
                || ()
            },
            (),
        );
    }
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
            <video style="max-height: 80vh; max-width: 80vw" controls=true src={format!("/vid/{}",(*video).filename.clone())} type="video/mp4"/>
            <div>
                <ul style="list-style-type: none;">
                    <li>
                        {(*video).date.clone()}
                    </li>
                </ul>
                </div>
            </div>
        </div>
    };
}
