use cobul::props::{Color, InputType, Size};
use cobul::*;
use validator::Validate;
use yew::prelude::*;

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
    let actions = Actions::new().submit(onsubmit.reform(|player: Player| player.name));
    let (form, Player { name: value }) = use_form(&player, actions);

    let onkeypress = Callback::from(move |e: KeyboardEvent| {
        if e.key() == "Enter" {
            onsubmit.emit(player.name.clone())
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
