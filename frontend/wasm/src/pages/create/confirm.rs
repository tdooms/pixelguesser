use yew::prelude::*;
use pbs::prelude::*;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub onconfirm: Callback<()>
}

#[function_component(Confirm)]
pub fn confirm(props: &Props) -> Html {
    html! {
        <Button onclick={props.onconfirm.clone()}> {"confirm"} </Button>
    }
}