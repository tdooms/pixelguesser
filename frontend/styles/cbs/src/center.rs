use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Center)]
pub fn center(props: &Props) -> Html {
    html! {
        <div class="columns is-centered is-desktop is-vcentered" style="height:100vh">
            <div class="column">
                { for props.children.iter() }
            </div>
        </div>
    }
}
