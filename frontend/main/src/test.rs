use cobul::*;
use yew::*;
use ywt::callback;

#[function_component(Test)]
pub fn test() -> Html {
    let state = use_state(|| false);
    let onclick = callback!(state; move |_| state.set(!*state));

    let inner = match *state {
        true => html! {
            <Buttons>
                <Button> <Icon icon={fa::Solid::Xmark}/> <span> {"Text1"} </span> </Button>
                <Button> <Icon icon={fa::Solid::Plus}/> <span> {"Text2"} </span> </Button>
            </Buttons>
        },
        false => html! {
            <Buttons>
                <Button> <Icon icon={fa::Solid::Eye}/> <span> {"Text3"} </span> </Button>
                <Button> <Icon icon={fa::Solid::Plus}/> <span> {"Text4"} </span> </Button>
            </Buttons>
        },
    };

    html! {
        <>
        {inner}
        <Box>
        <Button {onclick}> <Icon icon={fa::Solid::ArrowRight}/> <span> {"Swap"} </span> </Button>
        </Box>
        </>
    }
}
