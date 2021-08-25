use web_sys::Url;
use yew::prelude::*;

use pbs::prelude::*;
use pbs::properties::{Color, ColumnSize};
use cbs::SidebarAlignment;

use crate::graphql::DraftRound;

use super::{CenterImage, SideImages, RoundOptions, SideUpload};

pub enum Msg {
    AddRound,
    RemoveRound,
    ChangeRound(DraftRound),

    AddImage(Vec<web_sys::File>),
    RemoveImage,
    SelectImage(usize),
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub onback: Callback<()>,
    pub ondone: Callback<()>
}

pub struct CreateRounds {
    props: Props,
    link: ComponentLink<Self>,
    rounds: Vec<DraftRound>,
    current: usize,
}

impl Component for CreateRounds {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link, current: 0, rounds: vec![DraftRound::default()] }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangeRound(draft) => {
                self.rounds[self.current] = draft;
                false
            }
            Msg::RemoveRound => {
                self.rounds.remove(self.current);
                // TODO: I suspect the current index not always valid?
                true
            }
            Msg::AddRound => {
                self.current = self.rounds.len();
                self.rounds.push(DraftRound::default());
                true
            }
            Msg::RemoveImage => {
                self.rounds[self.current].image_local = None;
                self.rounds[self.current].image_url = None;
                true
            }
            Msg::AddImage(files) if files.len() == 1 => {
                false
            }
            Msg::AddImage(files) => {
                // TODO: error
                false
            }
            Msg::SelectImage(index) => {
                self.current = index;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let add_round = self.link.callback(|_| Msg::AddRound);
        let remove_round = self.link.callback(|_| Msg::RemoveRound);
        let change_round = self.link.callback(move |draft| Msg::ChangeRound(draft));

        let add_image = self.link.callback(|file| Msg::AddImage(file));
        let remove_image = self.link.callback(|_| Msg::RemoveImage);
        let select_image = self.link.callback(Msg::SelectImage);

        let side_images: Vec<_> = self
            .rounds
            .iter()
            .map(|round| {
                round.image_local.as_ref().map(|x| Url::create_object_url_with_blob(&x).unwrap())
            })
            .collect();

        let draft = self.rounds[self.current].clone();

        let src = match (&draft.image_url, &draft.image_local) {
            (Some(url), Some(_)) => Some(url.clone()), // TODO: error
            (Some(url), _) => Some(url.clone()),
            (_, Some(image)) => Url::create_object_url_with_blob(image).ok(),
            (None, None) => None
        };

        let center = match src {
            Some(src) => html! { <CenterImage src={src} onremove={remove_image}/> },
            None => html! { {"no image"} }
        };

        let left_footer = html! {
            <Buttons extra="mt-auto px-4 py-2">
                <Button fullwidth=true color={Color::Danger} light=true onclick={remove_round}>
                    <Icon icon={"fas fa-trash"}/> <span> {"remove round"} </span>
                </Button>
                <Button fullwidth=true color={Color::Success} light=true onclick={add_round}>
                    <Icon icon={"fas fa-plus"}/> <span> {"add round"} </span>
                </Button>
            </Buttons>
        };

        let right_footer = html! {
            <Buttons extra="mt-auto px-4 py-2">
                <Button fullwidth=true color={Color::Success} light=true onclick={self.props.ondone.clone()}>
                    <Icon icon={"fas fa-arrow-right"}/> <span> {"done"} </span>
                </Button>
                <Button  fullwidth=true color={Color::Danger} light=true onclick={self.props.onback.clone()}>
                    <Icon icon={"fas fa-arrow-left"}/> <span> {"back"} </span>
                </Button>
            </Buttons>
        };

        let right_side = match draft.image_url.is_some() || draft.image_local.is_some() {
            true => html! { <RoundOptions draft={draft} onchange={change_round} /> },
            false => html! { <SideUpload onupload={add_image} />}
        };

        html! {
            <Columns>
                <cbs::Sidebar size={ColumnSize::Is2} alignment={SidebarAlignment::Left} extra="p-0" overflow=false footer={left_footer}>
                    <SideImages images={side_images} onclick={select_image} current={self.current}/>
                </cbs::Sidebar>

                <Column size={ColumnSize::Is8}>
                    { center }
                </Column>

                <cbs::Sidebar size={ColumnSize::Is2} alignment={SidebarAlignment::Right} extra="p-0" footer={right_footer}>
                    { right_side }
                </cbs::Sidebar>
            </Columns>
        }
    }
}
