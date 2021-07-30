use api::*;
use pbs::Size;
use std::collections::HashMap;
use yew::prelude::*;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub onclick: Callback<u64>,
    pub players: HashMap<u64, Player>,
}

pub struct Master {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for Master {
    type Message = u64;
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
        let view_player = |id: u64, player: &Player| {
            let onclick = self.link.callback(move |_| id);
            html! { <cbs::IconButton outlined=true size={Size::Large} fullwidth=true onclick={onclick} text={player.name.clone()}/> }
        };

        let iter = self.props.players.iter();
        let players = iter.map(|(id, player)| view_player(*id, player));

        html! {
            <>
                { for players }
            </>
        }
    }
}
