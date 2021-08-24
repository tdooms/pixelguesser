use yew::prelude::*;
use yew::utils::NeqAssign;

use pbs::prelude::*;
use pbs::properties::{Alignment, Color};

use crate::graphql::{DraftRound, GuessChoices, PointChoices};

pub enum Msg {
    Upload(Vec<web_sys::File>),
    Answer(String),
    Points(PointChoices),
    Guesses(GuessChoices),
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub onchange: Callback<DraftRound>,
    pub onupload: Callback<web_sys::File>,

    pub draft: DraftRound,
}

pub struct SideOptions {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for SideOptions {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Upload(images) if images.len() == 1 => {
                self.props.draft.image_local = Some(images[0].clone());
                self.props.onupload.emit(images[0].clone());
            }
            Msg::Upload(images) => {
                // give error
            }
            Msg::Answer(answer) => {
                // Convert empty string to None
                self.props.draft.answer = Some(answer).filter(|x| !x.is_empty());
            }
            Msg::Points(points) => {
                self.props.draft.points = points;
            }
            Msg::Guesses(guesses) => {
                self.props.draft.guesses = guesses;
            }
        }
        self.props.onchange.emit(self.props.draft.clone());
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let onpoints = self.link.callback(Msg::Points);
        let onguess = self.link.callback(Msg::Guesses);
        let onupload = self.link.callback(Msg::Upload);

        let Props { draft, .. } = &self.props;

        if draft.image_local.is_some() || draft.image_url.is_some() {
            html! {
                <div class="p-6">
                    <cbs::SimpleField label="Answer">
                        <Input oninput={self.link.callback(Msg::Answer)} />
                    </cbs::SimpleField>
                    <cbs::SimpleField label="Points">
                        <cbs::KvButtons<PointChoices> value={draft.points} color={Color::Link} alignment={Alignment::Centered} onclick={onpoints} />
                    </cbs::SimpleField>
                    <cbs::SimpleField label="Guesses">
                        <cbs::KvButtons<GuessChoices> value={draft.guesses} color={Color::Link} alignment={Alignment::Centered} onclick={onguess} />
                    </cbs::SimpleField>
                </div>
            }
        } else {
            html! {
                <cbs::Center>
                    <File boxed=true alignment={Alignment::Centered} onupload={onupload} />
                </cbs::Center>
            }
        }
    }
}
