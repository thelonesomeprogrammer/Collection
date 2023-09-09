use yew::prelude::*;
mod pages;

use pages::home::Home;
use pages::not_found::NotFound;
use pages::vid::Vid;
use pages::player::Player;
use yew_router::prelude::*;
//Create the main app that will load all other Components

pub struct App {
    navbar_active: bool,
}

//Message enum that is used for managing the life cycle of Components
pub enum Msg {
    ToggleNavbar,
}


#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/videos")]
    Vid,
    #[at("/videos/:id")]
    Player {id: String},
    #[not_found]
    #[at("/404")]
    NotFound,
}


fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Home /> }
        }
        Route::Vid => {
            html!(<Vid />)
        }
        Route::Player { id } => {
            html!(<Player vid={id.clone()}/>)
        }
        _ => {
            html! { <NotFound /> }
        }
    }
}


//Implement the Component interface

impl Component for App {
    type Message = Msg;
    type Properties = ();

    //Create a new App
    fn create(_ctx: &Context<Self>) -> Self {
        App {
            navbar_active: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        //Creates The HTML that will show up in the browser.
        html!{
        <BrowserRouter>
            {self.view_nav(&ctx)}
            <main>
                <Switch<Route> render={switch} />
            </main>
        </BrowserRouter>
        }
    }
}

// Entry point for starting the Yew application
pub fn main() {
    //Create the logger
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    //Start the Yew framework
    yew::Renderer::<App>::new().render();
}

impl App {
    
    fn view_nav(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <nav class="navbar">
                <div class="navbar_container">
                    <ul class="navbar_menu">
                        <li class="topbar_item">
                            <Link<Route> to={Route::Home} classes="nav_link">
                                <img src="/data/images/logo.png" alt="the lonesome grogrammer logo" height="250" class="logo"/>
                            </Link<Route>>
                        </li>
                        <li class="topbar_item">
                            <Link<Route> to={Route::Home} classes="nav_link">
                                { "Home" }
                            </Link<Route>>
                        </li>
                        <li class="topbar_item">
                            <Link<Route> to={Route::Vid} classes="nav_link">
                                { "videos" }
                            </Link<Route>>
                        </li>
                        <li class="topbar_item">
                            <a href="https://github.com/thelonesomeprogrammer" class="nav_link">
                                {"Github"}
                            </a>
                        </li>
                    </ul>
                </div>
            </nav>
        }
    }
}
