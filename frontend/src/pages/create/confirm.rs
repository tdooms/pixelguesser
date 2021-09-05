use yew::prelude::*;

use pbs::prelude::*;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub onconfirm: Callback<()>,
    pub onback: Callback<()>,
}

#[function_component(Confirm)]
pub fn confirm(props: &Props) -> Html {
    html! {
        <Buttons>
            <Button onclick={props.onconfirm.clone()}> {"confirm"} </Button>
            <Button onclick={props.onback.clone()}> {"back"} </Button>
        </Buttons>
    }
}