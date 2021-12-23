use yew::prelude::*;

use cobul::props::Size;
use cobul::*;
use sessions::Player;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub onguess: Callback<String>,
    pub players: Vec<Player>,
}

#[function_component(PlayerList)]
pub fn player_list(props: &Props) -> Html {
    let view_player = |player: &Player| {
        let cloned = player.name.clone();
        let onclick = props.onguess.reform(move |_| cloned.clone());

        html! { <Button outlined=true size={Size::Large} fullwidth=true onclick={onclick}> {player.name.clone()} </Button> }
    };

    html! {
        { for props.players.iter().map(view_player) }
    }
}
