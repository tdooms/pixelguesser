use yew::prelude::*;

use api::RoundDiff;
use gloo_file::callbacks::FileReader;
use gloo_file::File;
use pbs::{Alignment, Color};
use yew::utils::NeqAssign;

pub enum Msg {
    Upload(Vec<File>),
    // Read(),
    Answer(String),
    Points(i64),
    Guesses(i64),
    Remove,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub onchange: Callback<RoundDiff>,
    pub draft: RoundDiff,
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
                //
            }
            Msg::Upload(images) => {
                // give error
            }
            Msg::Remove => {
                self.props.draft.image_bytes = None;
            }
            Msg::Answer(answer) => {
                // Convert empty string to None
                self.props.draft.answer = Some(answer).filter(|x| !x.is_empty());
            }
            Msg::Points(points) => {
                self.props.draft.points = Some(points);
            }
            Msg::Guesses(guesses) => {
                self.props.draft.guesses = Some(guesses);
            }
        }
        self.props.onchange.emit(self.props.draft.clone());
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let points_values: Vec<(_, _)> = (1..=5).map(|x| (x.to_string(), x)).collect();
        let guesses_values: Vec<(_, _)> = (1..=3).map(|x| (x.to_string(), x)).collect();

        let onupload = self.link.callback(Msg::Upload);
        let onremove = self.link.callback(|_| Msg::Remove);

        match &self.props.draft.image_bytes {
            Some(bytes) => html! {
                <>
                <div>
                    <cbs::SimpleField label="Answer">
                        <pbs::Input oninput={self.link.callback(Msg::Answer)} />
                    </cbs::SimpleField>
                    <cbs::SimpleField label="Points">
                        <cbs::KvButtons<i32> values={points_values} color={Color::Link} alignment={Alignment::Centered} />
                    </cbs::SimpleField>
                    <cbs::SimpleField label="Guesses">
                        <cbs::KvButtons<i32> values={guesses_values} color={Color::Link} alignment={Alignment::Centered} />
                    </cbs::SimpleField>
                </div>
                <cbs::IconButton icon="fas fa-trash" fullwidth=true color={Color::Danger} light=true text="remove image" onclick={onremove}/>
                </>
            },
            None => html! {
                <cbs::Center>
                    <pbs::File boxed=true alignment={Alignment::Centered} onupload={onupload} />
                </cbs::Center>
            },
        }
    }
}
