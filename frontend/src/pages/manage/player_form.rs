use cobul::props::{Color, InputType, Size};
use cobul::*;
use validator::Validate;
use yew::prelude::*;

#[derive(Default, Debug, Clone, PartialEq, Validate)]
pub struct PlayerName {
    #[validate(length(min = 1))]
    pub name: String,
}

#[function_component(PlayerForm)]
pub fn player_form(props: &Form<PlayerName>) -> Html {
    let _errors = props.errors();
    let PlayerName { name } = props.inner();

    html! {
        <Field grouped=true>
            <Control expanded=true>
                <Input oninput={props.field(|x| &mut x.name)} size={Size::Large} r#type={InputType::Text} placeholder={"eg. Alex"} value={name}/>
            </Control>
            <Control>
                <Button size={Size::Large} color={Color::Info} onclick={props.submit()}>
                    <Icon icon="fas fa-plus"> </Icon>
                </Button>
            </Control>
        </Field>
    }
}
