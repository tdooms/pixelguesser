use yew::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {"le test"}
}

pub fn main() {
    yew::Renderer::<App>::new().render();
}
