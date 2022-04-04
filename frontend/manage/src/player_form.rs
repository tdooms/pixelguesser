use cobul::props::{Color, InputType, Size};
use cobul::*;
use validator::Validate;
use yew::prelude::*;

use shared::callback;

#[derive(Debug, Clone, PartialEq, Validate)]
struct Player {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub onsubmit: Callback<String>,
}

#[function_component(PlayerForm)]
pub fn player_form(props: &Props) -> Html {
    let Props { onsubmit } = props.clone();
    let state = use_state(|| String::new());

    let player = Player { name: (*state).clone() };

    let onsubmit = callback!(state, onsubmit; move |player: Player| {
        state.set(String::new());
        onsubmit.emit(player.name);
    });

    let onchange = callback!(state; move |player: Player| state.set(player.name));
    let actions = Actions::new().submit(onsubmit.clone()).change(onchange);

    log::trace!("render player form {:?}", player);
    let (form, player) = use_form(&player, actions);
    let value = player.name.clone();

    let onkeypress = Callback::from(move |e: KeyboardEvent| {
        if e.key() == "Enter" {
            onsubmit.emit(player.clone());
        }
    });

    html! {
        <div {onkeypress}>
        <Field grouped=true>
            <Control expanded=true>
                <Input oninput={form.field(|x| &mut x.name)} size={Size::Large} r#type={InputType::Text} placeholder={"eg. Alex"} {value}/>
            </Control>
            <Control>
                <Button size={Size::Large} color={Color::Info} onclick={form.submit()}>
                    <Icon icon={Icons::Plus}> </Icon>
                </Button>
            </Control>
        </Field>
        </div>
    }
}
