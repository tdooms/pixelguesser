use cobul::props::{Color, InputType, Size};
use cobul::*;
use validator::Validate;
use yew::prelude::*;

#[derive(Default, Debug, Clone, PartialEq, Validate)]
pub struct PlayerName {
    #[validate(length(min = 1))]
    pub name: String,
}

#[derive(Clone, PartialEq)]
pub enum PlayerMsg {
    Change(PlayerName),
    Submit(PlayerName),
}

#[function_component(PlayerForm)]
pub fn player_form(props: &Form<PlayerName, PlayerMsg>) -> Html {
    let errors = props.errors();
    let name = props.values();

    html! {
        <Field grouped=true>
            <Control expanded=true>
                <Input oninput={props.field(|x| &mut x.name, PlayerMsg::Change)} size={Size::Large} r#type={InputType::Text} placeholder={"eg. Alex"}/>
            </Control>
            <Control>
                <Button size={Size::Large} color={Color::Info} onclick={props.callback(PlayerMsg::Submit)}>
                    <Icon icon="fas fa-plus"> </Icon>
                </Button>
            </Control>
        </Field>
    }
}
