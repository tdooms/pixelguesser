use cobul::*;
use std::rc::Rc;
use validator::Validate;
use yew::*;
use ywt::callback;

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

    let player = Rc::new(Player { name: (*state).clone() });

    let onsubmit = callback!(state, onsubmit; move |player: Rc<Player>| {
        state.set(String::new());
        onsubmit.emit(player.name.clone());
    });
    let onchange = callback!(state; move |player: Rc<Player>| {
        state.set(player.name.clone());
    });

    let actions = Actions::new().submit(onsubmit.clone()).change(onchange);

    let form = use_form(player.clone(), actions);
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
                    <Icon icon={fa::Solid::Plus}> </Icon>
                </Button>
            </Control>
        </Field>
        </div>
    }
}
