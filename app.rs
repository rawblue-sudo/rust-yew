use log::info;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

use crate::app::modules::Header;
use crate::app::root::Model as Root;

pub struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        info!("App component created");
        Model {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <>
                <div class="app">
                <div class="title">{ "Project" }</div>
                    <Header />
                    <Root />
                </div>
            </>
        }
    }
}