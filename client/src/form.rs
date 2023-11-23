use yew::prelude::*;

#[function_component]
pub fn Form() -> Html {
    // name
    let name_ref = NodeRef::default();
    let name_ref_outer = name_ref.clone();

    // price
    let price_ref = NodeRef::default();
    let price_ref_outer = price_ref.clone();

    // submit form data
    let onclick = Callback::from(move |_| gloo_console::log!("Button Clicked"));

    html! {
        <div class="container">
            <h2 class="title">{"Add a Product"}</h2>
            <div class="label-container">
                <label for="name" class="label">
                    {"Name "}
                    <input ref={name_ref_outer.clone()}
                        id="name"
                        class=""
                        type="text"
                    />
                </label>

                <label for="price" class="label">
                    {"Price "}
                    <input
                        ref={price_ref_outer.clone()}
                        id="price"
                        class=""
                        type="number"
                    />
                </label>
                <button {onclick} >{"Add Product"}</button>
              </div>

        </div>
    }
}
