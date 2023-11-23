use yew::prelude::*;
mod components;
mod form;

use components::Products;
use form::Form;

#[function_component]
fn App() -> Html {
    html! {
        <div class="container">
            <h1 class="title">{"Yew Product App"}</h1>
            <Form />
            <Products/>
        </div>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
