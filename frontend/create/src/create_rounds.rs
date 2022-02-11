use cobul::props::{Color, ColumnSize, SidebarAlignment};
use cobul::{Button, Buttons, Column, Columns, Icon, Icons, Sidebar};
use gloo::timers::callback::Timeout;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::consts::{CREATE_LONG_SAVE_TIME, CREATE_SHORT_SAVE_TIME};
use crate::utils::set_timer;
use crate::{Error, ErrorAgent};
use api::DraftRound;

use super::{CenterSpace, RoundForm, RoundInfo, RoundList};

#[derive(Debug)]
pub enum Msg {
    AddRound,
    DeleteRound,
    UpdateRound(RoundInfo),
    SelectRound(usize),

    UploadImage(Vec<web_sys::File>),
    RemoveImage,

    LongTimer,
    ShortTimer,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub rounds: Vec<DraftRound>,
    pub onback: Callback<()>,
    pub ondone: Callback<()>,
    pub onsave: Callback<Vec<DraftRound>>,
}

enum Timer {
    Long(Timeout),
    Short(Timeout),
    None,
}

pub struct CreateRounds {
    current: usize,
    local: Vec<DraftRound>,

    errors: Box<dyn Bridge<ErrorAgent>>,

    changes: u64,
    timer: Timer,
}

impl Component for CreateRounds {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let local = match ctx.props().rounds.len() {
            0 => vec![DraftRound::default()],
            _ => ctx.props().rounds.clone(),
        };

        let errors = ErrorAgent::bridge(Callback::noop());
        Self { current: 0, changes: 0, local, timer: Timer::None, errors }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let res = match msg {
            Msg::UpdateRound(info) => {
                self.local[self.current].answer = info.answer;
                self.local[self.current].points = info.points;
                self.local[self.current].guesses = info.guesses;
                true
            }
            Msg::DeleteRound if self.local.len() <= 1 => {
                self.errors.send(Error::DeleteOnlyRound);
                false
            }
            Msg::DeleteRound => {
                self.local.remove(self.current);
                self.current = if self.current == 0 { 0 } else { self.current - 1 };
                true
            }
            Msg::AddRound => {
                self.current = self.local.len();
                self.local.push(DraftRound::default());
                true
            }
            Msg::SelectRound(index) => {
                self.current = index;
                true
            }
            Msg::RemoveImage => {
                self.local[self.current].image = None;
                true
            }
            Msg::UploadImage(files) if files.len() == 1 => {
                self.local[self.current].image = api::Image::from_local(&files[0]);
                true
            }
            Msg::UploadImage(_files) => {
                self.errors.send(Error::MultipleFiles);
                false
            }
            Msg::ShortTimer => {
                log::error!("short timer");
                self.timer =
                    Timer::Long(set_timer(ctx.link(), CREATE_LONG_SAVE_TIME, Msg::LongTimer));

                if self.changes > 5 {
                    ctx.props().onsave.emit(self.local.clone());
                    self.changes = 0
                }
                false
            }
            Msg::LongTimer => {
                log::error!("long timer");
                ctx.props().onsave.emit(self.local.clone());
                self.timer = Timer::None;
                false
            }
        };

        if let (true, Timer::Long(_) | Timer::None) = (res, &self.timer) {
            self.changes += 1;
            log::error!("changes {}", self.changes);
            self.timer =
                Timer::Short(set_timer(ctx.link(), CREATE_SHORT_SAVE_TIME, Msg::ShortTimer));
        }

        res
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Also save stuff when changes are made
        let ondone = {
            let changes = self.changes;
            let onsave = ctx.props().onsave.clone();
            let rounds = self.local.clone();

            ctx.props().ondone.reform(move |_| {
                if changes > 0 {
                    onsave.emit(rounds.clone())
                }
            })
        };

        let round = &self.local[self.current];

        let left = {
            let images: Vec<_> =
                self.local.iter().map(|x| x.image.as_ref().map(api::Image::src)).collect();

            let onadd = ctx.link().callback(|_| Msg::AddRound);
            let ondelete = ctx.link().callback(|_| Msg::DeleteRound);
            let onselect = ctx.link().callback(Msg::SelectRound);

            html! {
                <Sidebar size={ColumnSize::Is2} alignment={SidebarAlignment::Left} class="p-0" overflow=true>
                    <RoundList {images} {onselect} {ondelete} {onadd} current={self.current}/>
                </Sidebar>
            }
        };

        let center = {
            let onupload = ctx.link().callback(Msg::UploadImage);
            let onremove = ctx.link().callback(|_| Msg::RemoveImage);

            html! { <CenterSpace image={round.image.clone()} {onremove} {onupload}/>}
        };

        let right = {
            let onback = ctx.props().onback.clone();
            let onchange = ctx.link().callback(Msg::UpdateRound);
            let round: RoundInfo = round.into();

            let footer = html! {
                <Buttons class="mt-auto px-4 py-2">
                    <Button fullwidth=true color={Color::Primary} onclick={ondone} light=true>
                        <Icon icon={Icons::ArrowRight}/> <span> {"Overview"} </span>
                    </Button>
                    <Button  fullwidth=true color={Color::Info} outlined=true onclick={onback}>
                        <Icon icon={Icons::ArrowLeft}/> <span> {"Edit Quiz"} </span>
                    </Button>
                </Buttons>
            };

            html! {
                <Sidebar size={ColumnSize::Is2} alignment={SidebarAlignment::Right} class="p-0" overflow=false footer={footer}>
                    <RoundForm inner={round.clone()} onchange={onchange} />
                </Sidebar>
            }
        };

        html! {
            <Columns>
                {left}
                <Column size={ColumnSize::Is8}> {center} </Column>
                {right}
            </Columns>
        }
    }
}
