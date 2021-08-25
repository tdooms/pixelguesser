use strum::IntoEnumIterator;
use yew::prelude::*;

use pbs::prelude::*;
use pbs::properties::{Alignment, Color, Size};
use shared::{Action, Stage};

use crate::route::Route;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub onchange: Callback<Stage>,
    pub stage: Stage,
    pub rounds: usize,
}

pub struct Navigate;

impl Component for Navigate {
    type Message = Action;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {  }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        if let Action::Leave = msg {
            yew_router::push_route(Route::Overview);
        }

        match ctx.props().stage.perform(msg, ctx.props().rounds) {
            Some(stage) => ctx.props().onchange.emit(stage),
            None => log::info!("faulty transition in navigation"),
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
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

        let actions = ctx.props().stage.actions(ctx.props().rounds);

        let map_action = |action: Action| {
            let (text, icon, color, light) = map_action_attrs(action);
            let callback = ctx.link().callback(move |_| action);
            let hidden = !actions.contains(&action);

            html! {
                <Button hidden={hidden} color={color} light={light} size={Size::Large} onclick={callback}>
                    <Icon icon={icon}/> <span>{text}</span>
                </Button>
            }
        };

        html! {
            <Buttons alignment={Alignment::Centered} extra="mt-4">
                { for Action::iter().map(map_action) }
            </Buttons>
        }
    }
}
