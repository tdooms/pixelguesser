use yew::prelude::*;

use cobul::props::Size;
use cobul::*;
use sessions::Player;
use std::collections::HashMap;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub onclick: Callback<String>,
    pub players: HashMap<String, Player>,
    pub title: String,
}

#[function_component(PlayerList)]
pub fn player_list(props: &Props) -> Html {
    let Props { onclick, players, title } = props;

    let view_player = |(name, _): (&String, &Player)| {
        let cloned = name.clone();
        let onclick = onclick.reform(move |_| cloned.clone());

        html! {
            <Button outlined=true size={Size::Large} fullwidth=true onclick={onclick}>
                {name.clone()}
            </Button>
        }
    };

    html! {
        <>
        { title.clone() }
        { for players.iter().map(view_player) }
        </>
    }
}
