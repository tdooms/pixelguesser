use std::collections::HashMap;
use std::rc::Rc;

use cobul::*;
use yew::*;

use api::{Player, Session};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub submit: Callback<HashMap<String, Player>>,
    pub session: Rc<Session>,
    pub title: AttrValue,
}

#[function_component(PlayerEdit)]
pub fn player_edit(props: &Props) -> Html {
    let Props { submit, session, title } = props.clone();
    let players = use_state(|| session.players.clone());

    let view_player = |(name, player): (&String, &Player)| {
        html! {
            <Columns vcentered=true centered=true>
            <Column size={ColumnSize::Is2}> <span class="has-text-centered"> {name} </span> </Column>
            <Column size={ColumnSize::IsNarrow}> <simple::Button class="mb-0" icon={fa::Solid::Plus} /> </Column>
            <Column size={ColumnSize::Is1}> <Input value={player.score.to_string()} disabled=true /> </Column>
            <Column size={ColumnSize::IsNarrow}> <simple::Button class="mb-0" icon={fa::Solid::Minus} /> </Column>
            </Columns>
        }
    };

    let title = match session.players.len() {
        0 => html! {},
        _ => html! {<Block class="has-text-centered"> { title.clone() } </Block>},
    };

    html! {
        <>
        { title }
        { for session.players.iter().map(view_player) }
        </>
    }
}
