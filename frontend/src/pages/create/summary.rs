use yew::prelude::*;

use cobul::*;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub onconfirm: Callback<()>,
    pub onback: Callback<()>,
}

#[function_component(Summary)]
pub fn summary(props: &Props) -> Html {
    html! {
        <>
        <p> {"This is a beautiful summary where the users can see what they've \
        made and maybe still change some settings such as public/private"} </p>

        <Buttons>
            <Button onclick={props.onconfirm.clone()}> {"confirm"} </Button>
            <Button onclick={props.onback.clone()}> {"back"} </Button>
        </Buttons>
        </>
    }
}
