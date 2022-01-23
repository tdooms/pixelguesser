use cobul::props::ColumnSize;
use cobul::*;
use gloo::timers::callback::Timeout;
use yew::prelude::*;

use crate::graphql::{DraftRound, ImageData};

use super::{CenterSpace, LeftBar, RightBar, RoundInfo};

#[derive(Debug)]
pub enum Msg {
    AddRound,
    RemoveRound,
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
        Self { current: 0, changes: 0, local, timer: Timer::None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let res = match msg {
            Msg::UpdateRound(info) => {
                self.local[self.current].answer = info.answer;
                self.local[self.current].points = info.points;
                self.local[self.current].guesses = info.guesses;
                true
            }
            Msg::RemoveRound if self.local.len() <= 1 => {
                // TODO: give error
                false
            }
            Msg::RemoveRound => {
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
                self.local[self.current].image = ImageData::from_local(&files[0]);
                true
            }
            Msg::UploadImage(_files) => {
                // TODO: give error
                false
            }
            Msg::ShortTimer => {
                let cloned = ctx.link().clone();
                self.timer =
                    Timer::Long(Timeout::new(10_000, move || cloned.send_message(Msg::LongTimer)));

                if self.changes > 5 {
                    ctx.props().onsave.emit(self.local.clone());
                    self.changes = 0
                }
                false
            }
            Msg::LongTimer => {
                ctx.props().onsave.emit(self.local.clone());
                self.timer = Timer::None;
                false
            }
        };

        // The timing works as follows: we only submit an update when
        // - either the long timer passes
        // - enough changes have been made

        // The short timer is there to make sure quick
        // spamming changes doesn't cause too many submits
        if let (true, Timer::Long(_) | Timer::None) = (res, &self.timer) {
            self.changes += 1;
            let cloned = ctx.link().clone();
            self.timer =
                Timer::Short(Timeout::new(1_000, move || cloned.send_message(Msg::ShortTimer)))
        }

        res
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onback = ctx.props().onback.clone();

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

        let add_round = ctx.link().callback(|_| Msg::AddRound);
        let remove_round = ctx.link().callback(|_| Msg::RemoveRound);
        let select_round = ctx.link().callback(Msg::SelectRound);
        let update_round = ctx.link().callback(Msg::UpdateRound);

        let upload_image = ctx.link().callback(Msg::UploadImage);
        let remove_image = ctx.link().callback(|_| Msg::RemoveImage);

        let images: Vec<_> =
            self.local.iter().map(|round| round.image.as_ref().map(ImageData::src)).collect();

        let round = &self.local[self.current];

        let info = match round.image {
            Some(_) => Some(round.into()),
            None => None,
        };

        html! {
            <Columns>
                <LeftBar images={images} current={self.current} size={ColumnSize::Is2}
                         onremove={remove_round} onadd={add_round} onselect={select_round}/>

                <Column size={ColumnSize::Is8}>
                    <CenterSpace image={round.image.clone()} onremove={remove_image} onupload={upload_image}/>
                </Column>

                <RightBar round={info} size={ColumnSize::Is2} ondone={ondone} onback={onback} onchange={update_round}/>
            </Columns>
        }
    }
}
