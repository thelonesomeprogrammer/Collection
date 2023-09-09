use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;
use yew_router::prelude::Link;
use crate::Route;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement, InputEvent, HtmlElement, HtmlSelectElement};

use regex::Regex;

use rand::thread_rng;
use rand::seq::SliceRandom;



#[derive(Deserialize, PartialEq,Clone,Debug)]
pub struct Video {
    pub id:String,
    pub filename:String,
    pub title:String,
    pub url:String,
    pub date:String,
    pub plot:String,
    pub img:String,
}
#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
}

#[function_component(VideosList)]
fn videos_list(VideosListProps { videos }: &VideosListProps) -> Html {
    videos.iter().map(|video| html! {
        <div style="position:relative;">
            <Link<Route>  key={video.id.clone()} to={Route::Player { id: video.id.as_str().replace("scene:", "") }}> 
                <img style = "object-fit: cover; width: 24.7vw; height: 13.78vw" loading="lazy" src={video.img.clone()} alt={video.filename.clone()}/>
                <div style = "position: absolute; width: 24.7vw; height: 13.78vw; top: 0; right: 0; background: radial-gradient(closest-side, rgba(0,0,0,0) 0%,rgba(0,0,0,0) 60%, rgba(0,0,0,0.3) 90%, rgba(0,0,0,0.4) 100%);" class = "overleg">
                    {video.title.clone()}
                </div>
            </Link<Route>>
        </div> 
    }).collect()
}


#[derive(Properties, PartialEq)]
struct PagerProps{
  //  oninc:Callback<MouseEvent>,
  //  ondec:Callback<MouseEvent>,
    page:usize,
    newpage:Callback<MouseEvent>,
    count:Vec<usize>,
}

#[function_component(Pager)]
fn pager(PagerProps { count, newpage, page }: &PagerProps) -> Html {
    let style1 = "width: 40px; height: 40px; font-size: xx-large; color: white; background-color: #24242499; border-radius: 10px;";
    let style2 = "width: 40px; height: 40px; font-size: xx-large; color: white; background-color: #242424d4; border-radius: 10px;";
    count.iter().map(|i| 
        html!{
            <li>
                <button onclick = {newpage} style = {if i == page {style1} else {style2}} >{i}</button>
            </li>
    }).collect()
}

fn get_value_from_mouse_event(e: MouseEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlElement = event_target.dyn_into().unwrap_throw();
    target.text_content().unwrap().to_string()
}

fn get_value_from_select(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlSelectElement = event_target.dyn_into().unwrap_throw();
    target.value()
}

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    web_sys::console::log_1(&target.value().into());
    target.value()
}
fn stringify (x:Vec<Video>) -> String {
    let mut res = "".to_string();
    for i in x.iter(){
        let astr = format!("●{} {} {} {}●",i.id,i.title,i.date,i.filename).to_lowercase();
        res.push_str(astr.as_str());
    };
    res
}


#[function_component(Vid)]
pub fn vid() -> Html {
    let value = use_state(|| "".to_string());
    let page = use_state(|| 0);
    let showcaunt = use_state(|| 20);

    let videos = use_state(|| vec![]);
    let vidfil = use_state(|| vec![]);
    let vidshow = use_state(|| vec![]);

    {
        let videos = videos.clone();
        let vidfil = vidfil.clone();
        let vidshow = vidshow.clone();
        use_effect_with_deps(
            move |_| {
                let videos = videos.clone();
                let vidfil = vidfil.clone();
                let vidshow = vidshow.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_videos: Vec<Video> = Request::get("/matadata/")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    let mut rvid = fetched_videos.to_vec();
                    rvid.shuffle(&mut thread_rng());
                    videos.set(rvid.clone());
                    vidfil.set(rvid.clone());
                    vidshow.set(if rvid.len() > 20 { rvid[(0)..20].to_vec()} else {rvid});
                });
                || ()
            },
            (),
        );
    }

    let oninc = {
        let count = page.clone();
        let showcaunt = *showcaunt;
        let page = *count+1;
        let vidfil = vidfil.clone();
        let vidshow = vidshow.clone();

        Callback::from(move |_| {
            web_sys::console::log_1(&page.into());
            if (*vidfil).len() > showcaunt*page {
            count.set(page);

            let new: Vec<Video> = (*vidfil).clone().to_vec();
            let new = if new.len() - showcaunt * page > showcaunt {new[(page * showcaunt)..((page + 1) * showcaunt)].to_vec()} else {new[(page * showcaunt)..(new.len()-1)].to_vec()};
            vidshow.set(new)
        }})
    };

    let newpage = {
        let count = page.clone();
        let showcaunt = *showcaunt;
        let vidfil = vidfil.clone();
        let vidshow = vidshow.clone();

        Callback::from(move |e| {
            let r = get_value_from_mouse_event(e);
            web_sys::console::log_1(&r.clone().into());
            let page = r.clone().parse::<usize>().unwrap()-1;
            count.set(page);
            let new: Vec<Video> = (*vidfil).clone().to_vec();
            let new = if new.len() - showcaunt * page > showcaunt {new[(page * showcaunt)..((page + 1) * showcaunt)].to_vec()} else {new[(page * showcaunt)..(new.len()-1)].to_vec()};
                vidshow.set(new)
        })
    };

    let ondec = {
        let count = page.clone();
        let showcaunt = *showcaunt;
        let page = *count;
        let vidfil = vidfil.clone();
        let vidshow = vidshow.clone();

        Callback::from(move |_| {
            web_sys::console::log_1(&page.into());
            if page > 0 {
                let page = page-1;
                count.set(page);
                let new: Vec<Video> = (*vidfil).clone().to_vec()[(page * showcaunt)..((page+1) * showcaunt)].to_vec();
                vidshow.set(new)
            }
        })
    };

    let on_new_show = {
        let showcaunt = showcaunt.clone();
        let count = page.clone();
        let vidfil = (*vidfil).len();

        Callback::from(move |e| {
            let temp = get_value_from_select(e);
            let num :usize = temp.parse().unwrap();  
            if vidfil < num + num * *count {count.set(vidfil/(num))};
            showcaunt.set(num);

        })
    };

    let oninput = {
        let vidshow = vidshow.clone();
        let vidfil = vidfil.clone();
        let val = value.clone();
        let showcaunt = showcaunt.clone();
        let page = page.clone();


        Callback::from(move |input_event: InputEvent| {
            let temp = get_value_from_input_event(input_event).to_lowercase();
            let tab = stringify((*videos).clone());

            let re = Regex::new(r"●(?P<id>[\w+|:]*) [\w+| |/|\-|\.|:|'|&]*ins[\w+| |/|\-|\.|:|'|&]*●".replace("ins",temp.as_str()).as_str(),);

            let mut map = Vec::new();
            for mat in re.unwrap().captures_iter(tab.as_str()) {
                map.push(mat.name("id").unwrap().as_str().to_string());
            }

            let new: Vec<Video> = (videos).clone().to_vec().into_iter().filter(|x| map.contains(&x.id)).collect();
            vidfil.set(new.clone());
            let new = if new.len() > *showcaunt { new[0..(*showcaunt)].to_vec()} else {new};
            vidshow.set(new);
            page.set(0);
            val.set(temp)
        })
    };
    return html! {
        <div style = "">
            <div class="search-container" style="padding-top: 100px; justify-content: center; display:flex;" >
                    <input style="font-size: xx-large; font-family: 'Roboto', sans-serif; padding: 10px; border: none; border-radius: 40px; box-shadow: 0px 5px 15px -3px #cccccc;" type="text" placeholder="Search.." value = {(*value).clone()} {oninput} />
                      <select style="font-size: xx-large; font-family: 'Roboto', sans-serif; padding: 10px; border: none; border-radius: 40px; box-shadow: 0px 5px 15px -3px #cccccc;" oninput={on_new_show}>
                        <option value=100 >{"100"}</option>
                        <option value=80 >{"80"}</option>
                        <option value=60 >{"60"}</option>
                        <option value=40 >{"40"}</option>
                        <option value=30 >{"30"}</option>
                        <option value=20 >{"20"}</option>
                    </select>
            </div>
            <div style="padding-top:50px; justify-content: space-around; grid-template-columns: auto auto auto auto; row-gap: 0.125vw; display:grid; justify-items:center;" >
            <VideosList videos={if (*vidfil).len() > 0 {if (*vidfil).len() > *showcaunt  + *showcaunt * *page{(*vidfil)[(*page * *showcaunt)..((*page + 1) * *showcaunt)].to_vec()} else {(*vidfil)[(*page * *showcaunt)..((*vidfil).len()-1)].to_vec()}} else {vec![]}} />
            </div>
            <div style="justify-content: center;display:flex;align-content: center;height: 50px;padding: 5px;" >
                <ul style = "display: flex; text-decoration: none; list-style: none; flex-wrap: wrap; align-content: center;" >
                    <li>
                        <button style="width: 40px; height: 40px; font-size: xx-large; color: white; background-color: #242424d4; border-radius: 10px;" onclick = {ondec}>{ "<" }</button>
                    </li>
                    <Pager newpage = {newpage} page = {*page} count = {(1..((*vidfil).len()/(*showcaunt)+1+{if (*vidfil).len()%(*showcaunt)>0{1}else{0}})).collect::<Vec<usize>>()}/>
                    <li>
                        <button style="width: 40px; height: 40px; font-size: xx-large; color: white; background-color: #242424d4; border-radius: 10px;" onclick = {oninc}>{ ">" }</button>
                    </li>
                </ul>
            </div>
        </div>
    };
}
