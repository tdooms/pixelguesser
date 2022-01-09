use cobul::props::{Color, ColumnSize, SidebarAlignment};
use cobul::*;
use yew::prelude::*;

use crate::graphql::DraftRound;
use crate::structs::ImageData;

use super::{CenterImage, SideImages, SideInfo, SideUpload};

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
    pub ondone: Callback<()>,
}

pub struct CreateRounds {
    rounds: Vec<DraftRound>,
    current: usize,
}

impl Component for CreateRounds {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { current: 0, rounds: vec![DraftRound::default()] }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangeRound(round) => {
                self.rounds[self.current] = round;
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
                self.rounds[self.current].image = None;
                true
            }
            Msg::AddImage(files) if files.len() == 1 => {
                self.rounds[self.current].image = ImageData::from_local(&files[0]);
                true
            }
            Msg::AddImage(_files) => {
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
        let change_round_info = ctx.link().callback(move |draft| Msg::ChangeRound(draft));

        let add_image = ctx.link().callback(|file| Msg::AddImage(file));
        let remove_image = ctx.link().callback(|_| Msg::RemoveImage);
        let select_image = ctx.link().callback(Msg::SelectImage);

        let side_images: Vec<_> =
            self.rounds.iter().map(|round| round.image.as_ref().map(ImageData::src)).collect();

        let draft = self.rounds[self.current].clone();

        let center = match draft.image.as_ref().map(ImageData::src) {
            Some(src) => html! { <CenterImage src={src} onremove={remove_image}/> },
            None => html! { <Center> {"no image"} </Center> },
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

        let right_side = match draft.image.as_ref().map(ImageData::src) {
            Some(_) => html! { <SideInfo info={draft.clone()} onchange={change_round_info} /> },
            None => html! { <SideUpload onupload={add_image} />},
        };

        html! {
            <Columns>
                <Sidebar size={ColumnSize::Is2} alignment={SidebarAlignment::Left} extra="p-0" overflow=false footer={left_footer}>
                    <SideImages images={side_images} onclick={select_image} current={self.current}/>
                </Sidebar>

                <Column size={ColumnSize::Is8}>
                    { center }
                </Column>

                <Sidebar size={ColumnSize::Is2} alignment={SidebarAlignment::Right} extra="p-0" footer={right_footer}>
                    { right_side }
                </Sidebar>
            </Columns>
        }
    }
}
