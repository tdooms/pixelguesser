use yew::prelude::*;

use cobul::props::Size;
use cobul::*;
use sessions::Player;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub onclick: Callback<String>,
    pub players: Vec<Player>,
    pub title: String,
}

#[function_component(PlayerList)]
pub fn player_list(props: &Props) -> Html {
    let Props { onclick, players, title } = props;

    let view_player = |player: &Player| {
        let cloned = player.name.clone();
        let onclick = onguess.reform(move |_| cloned.clone());

        html! {
            <Button outlined=true size={Size::Large} fullwidth=true onclick={onclick}>
                {player.name.clone()}
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
