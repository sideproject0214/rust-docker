use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    id: i32,
    name: String,
    price: i32,
}

#[function_component]
pub fn Products() -> Html {
    let data: UseStateHandle<Vec<Product>> = use_state(|| vec![]);

    {
        let data_clone = data.clone();
        use_effect(move || {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_data = reqwest::get("http://127.0.0.1/api/products")
                    .await
                    .expect("cannot get data from url")
                    .json::<Vec<Product>>()
                    .await
                    .expect("cannot convert to json");

                data_clone.set(fetched_data);
            });
            || ()
        })
    }

    let products = data
        .iter()
        .map(|product| {
            html! {
                <ul>
                    <li key={product.id}>
                        {format!("Name: {}, Price: {}", product.name, product.price )}
                    </li>
                </ul>
            }
        })
        .collect::<Html>();

    html! {
        <div class="container">
            <h2>{"List of Products: "} {data.len()}</h2>
            <p>{products}</p>
        </div>
    }
}
