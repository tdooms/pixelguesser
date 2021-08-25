
use yew::prelude::*;

use cbs::SidebarAlignment;
use pbs::prelude::*;
use pbs::properties::{Color, ColumnSize};

use crate::graphql::{DraftRound, RoundInfo, Image};

use super::{CenterImage, SideInfo, SideImages, SideUpload};

pub enum Msg {
    AddRound,
    RemoveRound,
    ChangeRoundInfo(RoundInfo),

    AddImage(Vec<web_sys::File>),
    RemoveImage,
    SelectImage(usize),
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub onback: Callback<()>,
    pub ondone: Callback<()>,
}

pub struct CreateRounds {
    rounds: Vec<DraftRound>,
    current: usize,
}

impl Component for CreateRounds {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self { current: 0, rounds: vec![DraftRound::default()] }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangeRoundInfo(info) => {
                self.rounds[self.current].info = info;
                true
            }
            Msg::RemoveRound => {
                self.rounds.remove(self.current);
                // TODO: I suspect the current index is not always valid?
                true
            }
            Msg::AddRound => {
                self.current = self.rounds.len();
                self.rounds.push(DraftRound::default());
                true
            }
            Msg::RemoveImage => {
                self.rounds[self.current].image = Image::None;
                true
            }
            Msg::AddImage(files) if files.len() == 1 => {
                self.rounds[self.current].image = Image::new(&files[0]);
                true
            }
            Msg::AddImage(files) => {
                // TODO: give error
                false
            }
            Msg::SelectImage(index) => {
                self.current = index;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let add_round = ctx.link().callback(|_| Msg::AddRound);
        let remove_round = ctx.link().callback(|_| Msg::RemoveRound);
        let change_round_info = ctx.link().callback(move |draft| Msg::ChangeRoundInfo(draft));

        let add_image = ctx.link().callback(|file| Msg::AddImage(file));
        let remove_image = ctx.link().callback(|_| Msg::RemoveImage);
        let select_image = ctx.link().callback(Msg::SelectImage);

        let side_images: Vec<_> = self
            .rounds
            .iter()
            .map(|round| round.image.src())
            .collect();

        let draft = self.rounds[self.current].clone();

        let center = match draft.image.src() {
            Some(src) => html! { <CenterImage src={src} onremove={remove_image}/> },
            None => html! { <cbs::Center> {"no image"} </cbs::Center> }
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
                <Button fullwidth=true color={Color::Success} light=true onclick={ctx.props().ondone.clone()}>
                    <Icon icon={"fas fa-arrow-right"}/> <span> {"done"} </span>
                </Button>
                <Button  fullwidth=true color={Color::Danger} light=true onclick={ctx.props().onback.clone()}>
                    <Icon icon={"fas fa-arrow-left"}/> <span> {"back"} </span>
                </Button>
            </Buttons>
        };

        let right_side = match draft.image.src() {
            Some(_) => html! { <SideInfo info={draft.info} onchange={change_round_info} /> },
            None => html! { <SideUpload onupload={add_image} />}
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
