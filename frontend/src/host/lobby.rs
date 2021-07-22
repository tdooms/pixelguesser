use yew::prelude::*;
use yewtil::NeqAssign;

use api::Session;
use pbs::{Color, ColumnSize, HeroSize};

use crate::utils::code_to_string;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct LobbyProps {
    pub session: Session,
    pub session_id: u64,
    pub has_manager: bool,
}

pub struct Lobby {
    props: LobbyProps,
}

impl Component for Lobby {
    type Message = ();
    type Properties = LobbyProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let players = self.props.session.players.iter().map(|(_, player)| {
            html! {
                <pbs::Column size={ColumnSize::IsNarrow}>
                    <pbs::Box> {&player.name} </pbs::Box>
                </pbs::Column>
            }
        });

        let title = self.props.session.quiz.name.clone();

        let subtitle = match self.props.has_manager {
            true => "quiz master present",
            false => "no quiz master",
        };

        let code = code_to_string(self.props.session_id).unwrap_or_default();

        let body = html! {
            <pbs::Container extra="has-text-centered">
                <pbs::Title> {code} </pbs::Title>
            </pbs::Container>
        };

        html! {
            <>
                <pbs::SimpleHero title={title} subtitle={subtitle} />
                <pbs::Hero color={Color::Primary} size={HeroSize::Medium} body={body} />

                <pbs::Columns multiline=true centered=true extra="mt-5">
                    { for players }
                </pbs::Columns>
            </>
        }
    }
}
