use yew::prelude::*;

use api::{Action, Stage};
use pbs::{Alignment, Color, Size};

use crate::route::Route;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub onchange: Callback<Stage>,
    pub stage: Stage,
    pub rounds: usize,
}

pub struct Navigate {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for Navigate {
    type Message = Action;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        if let Action::Leave = msg {
            yew_router::push_route(Route::Overview);
        }

        match self.props.stage.perform(msg, self.props.rounds) {
            Some(stage) => self.props.onchange.emit(stage),
            None => log::info!("faulty transition in navigation"),
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let map_action_attrs = |action: Action| match action {
            Action::Start => ("start", "fas fa-play", Color::Primary, false),
            Action::Pause => ("pause", "fas fa-pause", Color::Light, false),
            Action::Resume => ("resume", "fas fa-play", Color::Light, false),
            Action::Reveal => ("reveal", "fas fa-eye", Color::Danger, true),
            Action::Scores => ("scores", "fas fa-list-ol", Color::Link, true),
            Action::Next => ("next", "fas fa-forward", Color::Success, false),
            Action::Finish => ("finish", "fas fa-flag-checkered", Color::Success, true),
            Action::Leave => ("leave", "fas fa-sign-out-alt", Color::Danger, true),
        };

        static ACTIONS: [Action; 8] = [
            Action::Start,
            Action::Pause,
            Action::Resume,
            Action::Reveal,
            Action::Scores,
            Action::Next,
            Action::Finish,
            Action::Leave,
        ];

        let actions = self.props.stage.actions(self.props.rounds);

        let map_action = |action: Action| {
            let (text, icon, color, light) = map_action_attrs(action);
            let callback = self.link.callback(move |_| action);
            let hidden = !actions.contains(&action);

            html! { <cbs::IconButton hidden={hidden} text={text} icon={icon} color={color}
            light={light} size={Size::Large} onclick={callback}/> }
        };

        html! {
            <pbs::Buttons alignment={Alignment::Centered} extra="mt-4">
                { for ACTIONS.iter().copied().map(map_action) }
            </pbs::Buttons>
        }
    }
}
