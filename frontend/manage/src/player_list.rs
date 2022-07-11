use yew::*;

use api::{Player, Session};
use cobul::*;
use std::rc::Rc;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub onclick: Callback<String>,
    pub session: Rc<Session>,
    pub title: String,
}

#[function_component(PlayerList)]
pub fn player_list(props: &Props) -> Html {
    let Props { onclick, session, title } = props;

    let view_player = |(name, _): (&String, &Player)| {
        let cloned = name.clone();
        let onclick = onclick.reform(move |_| cloned.clone());

        html! {
            <Button outlined=true size={Size::Large} fullwidth=true {onclick}>
                {name.clone()}
            </Button>
        }
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
