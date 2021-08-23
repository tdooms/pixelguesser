use yew::prelude::*;

use pbs::prelude::*;
use pbs::properties::Size;
use shared::Player;
use yew::utils::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub onguess: Callback<String>,
    pub players: Vec<Player>,
}

pub struct Master {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for Master {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, _: Self::Message) -> bool {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let view_player = |player: &Player| {
            let cloned = player.name.clone();
            let onclick = self.props.onguess.reform(move |_| cloned.clone());
            html! { <Button outlined=true size={Size::Large} fullwidth=true onclick={onclick}> {player.name.clone()} </Button> }
        };

        html! {
            { for self.props.players.iter().map(view_player) }
        }
    }
}
