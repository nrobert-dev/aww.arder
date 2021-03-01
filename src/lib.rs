#![recursion_limit="600"]

mod pages;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::ConsoleService;

mod app_router;
use app_router::AppRouter;

struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender { false }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <AppRouter/>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}