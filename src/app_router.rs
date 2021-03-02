use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages;


pub type Link = RouterAnchor<AppRoute>;
#[derive(Switch,Debug,Clone)]
pub enum AppRoute{
    #[to = "/awards/{team_name}"]
    Awards(String),
    #[to = "/"]
    Index
}

pub struct AppRouter {}
impl Component for AppRouter {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let render_func = Router::render(|route : AppRoute| match route {
                AppRoute::Index => html! { <pages::Home/>},
                AppRoute::Awards(team_name) => html! { <pages::Awards team_name={team_name}/>},
        });

        html! {
            <Router<AppRoute, ()> render=render_func/>
        }
    }
}