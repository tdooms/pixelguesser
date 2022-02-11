use cobul::props::{Color, InputType, Size};
use cobul::*;
use validator::Validate;
use yew::prelude::*;

#[derive(Default, Debug, Clone, PartialEq, Validate)]
pub struct PlayerName {
    pub name: String,
}

#[function_component(PlayerForm)]
pub fn player_form(props: &Form<PlayerName>) -> Html {
    let PlayerName { name } = props.inner();

    let cloned = props.onsubmit();
    let onkeypress = Callback::from(move |e: KeyboardEvent| {
        if e.key() == "Enter" {
            cloned.emit(())
        }
    });

    html! {
        <div {onkeypress}>
        <Field grouped=true>
            <Control expanded=true>
                <Input oninput={props.onfield(|x| &mut x.name)} size={Size::Large} r#type={InputType::Text} placeholder={"eg. Alex"} value={name}/>
            </Control>
            <Control>
                <Button size={Size::Large} color={Color::Info} onclick={props.onsubmit()}>
                    <Icon icon={Icons::Plus}> </Icon>
                </Button>
            </Control>
        </Field>
        </div>
    }
}
