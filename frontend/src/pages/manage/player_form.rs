use cobul::props::{Color, InputType, Size};
use cobul::*;
use validator::Validate;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Validate)]
struct PlayerName {
    #[validate(length(min = 1))]
    name: String,
}

#[function_component(PlayerForm)]
pub fn player_form(props: &Form<PlayerName, String>) -> Html {
    let errors = props.errors();
    let name = props.values();

    html! {
        <Field grouped=true>
            <Control expanded=true>
                <Input oninput={props.field(|x, v| x.name = v, |x| x)} size={Size::Large} r#type={InputType::Text} placeholder={"eg. Alex"}/>
            </Control>
            <Control>
                <Button size={Size::Large} color={Color::Info} onclick={props.callback(|x| x.name)}>
                    <Icon icon="fas fa-plus"> </Icon>
                </Button>
            </Control>
        </Field>
    }
}
