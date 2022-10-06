use std::rc::Rc;

use cobul::*;
use yew::*;

use api::{Player, Session};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub click: Callback<String>,
    pub session: Rc<Session>,
    pub title: AttrValue,
}

#[function_component(PlayerList)]
pub fn player_list(props: &Props) -> Html {
    let Props { click, session, title } = props.clone();

    let view_player = |(name, _): (&String, &Player)| {
        let click = ywt::callback!(name, click; move |_| click.emit(name.clone()));
        html! { <simple::Button outlined=true size={Size::Large} fullwidth=true {click} text={name.clone()} /> }
    };

    let title = match session.players.len() {
        0 => html! {},
        _ => html! {<Block class="has-text-centered"> { title.clone() } </Block>},
    };

    html! {
        <>
        { title }

        <Buttons>
            { for session.players.iter().map(view_player) }
        </Buttons>
        </>
    }
}
