use yew::prelude::*;
use yew::web_sys::{File as SysFile, Url};
use yewtil::NeqAssign;

use api::DraftRound;
use pbs::{Alignment, Color};

pub enum Msg {
    Upload(Vec<SysFile>),
    Answer(String),
    Points(i64),
    Guesses(i64),
    Remove,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub onchange: Callback<DraftRound>,
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
            Msg::Upload(images) => {
                self.props.draft.image_url = images
                    .iter()
                    .next()
                    .as_ref()
                    .map(|file| Url::create_object_url_with_blob(file).unwrap())
            }
            Msg::Remove => self.props.draft.image_url = None,
            Msg::Answer(answer) => {
                if answer.is_empty() {
                    self.props.draft.answer = None
                } else {
                    self.props.draft.answer = Some(answer)
                }
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
        let points_values: Vec<(_, _)> = (1..=5).map(|x| (x.to_string(), x)).collect();
        let guesses_values: Vec<(_, _)> = (1..=3).map(|x| (x.to_string(), x)).collect();

        let answer_label = html_nested! { <pbs::Label> {"Answer"} </pbs::Label> };
        let answer_inner = html! { <pbs::Input oninput={self.link.callback(Msg::Answer)} /> };

        let points_label = html_nested! { <pbs::Label> {"Points"} </pbs::Label> };
        let points_inner = html! { <pbs::KvButtons<i32> values={points_values} color={Color::Link} alignment={Alignment::Centered} /> };

        let guesses_label = html_nested! { <pbs::Label> {"Guesses"} </pbs::Label> };
        let guesses_inner = html! { <pbs::KvButtons<i32> values={guesses_values} color={Color::Link} alignment={Alignment::Centered} /> };

        let onupload = self.link.callback(Msg::Upload);
        let onremove = self.link.callback(|_| Msg::Remove);

        match &self.props.draft.image_url {
            Some(image) => html! {
                <>
                <div>
                    <pbs::Field label={answer_label}>
                        <pbs::Control inner={answer_inner} />
                    </pbs::Field>
                    <pbs::Field label={points_label}>
                        <pbs::Control inner={points_inner} />
                    </pbs::Field>
                    <pbs::Field label={guesses_label}>
                        <pbs::Control inner={guesses_inner} />
                    </pbs::Field>
                </div>
                <pbs::Button icon="fas fa-trash" fullwidth=true color={Color::Danger} light=true text="remove image" onclick={onremove}/>
                </>
            },
            None => html! {
                <pbs::Center>
                    <pbs::File boxed=true alignment={Alignment::Centered} onupload={onupload} />
                </pbs::Center>
            },
        }
    }
}
