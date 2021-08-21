use std::collections::HashMap;

use yew::prelude::*;

use pbs::Size;
use shared::Player;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub onguess: Callback<String>,
    pub players: Vec<Player>,
}

pub struct Master {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for Master {
    type Message = String;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        self.props.onclick.emit(msg);
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let view_player = |player: &Player| {
            let onclick = self.link.callback(move |_| player.name.clone());
            html! { <cbs::IconButton outlined=true size={Size::Large} fullwidth=true onclick={onclick} text={player.name.clone()}/> }
        };

        html! {
            { for self.props.players.iter().map(view_player) }
        }
    }
}
