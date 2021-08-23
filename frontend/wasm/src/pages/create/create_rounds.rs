use pbs::prelude::*;
use pbs::properties::{Color, ColumnSize};
use web_sys::Url;
use yew::prelude::*;

use super::{CenterImage, SideImages, SideOptions};
use crate::graphql::DraftRound;

pub enum Msg {
    Change(usize, DraftRound),
    Upload(usize, web_sys::File),
    Delete(usize),
    Clicked(usize),
    Remove,
    Add,
}

pub struct CreateRounds {
    link: ComponentLink<Self>,
    rounds: Vec<DraftRound>,
    current: usize,
}

impl Component for CreateRounds {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, current: 0, rounds: vec![DraftRound::default()] }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Change(index, draft) => {
                self.rounds[index] = draft;
            }
            Msg::Remove => {
                self.rounds.remove(self.current);
            }
            Msg::Add => {
                self.current = self.rounds.len();
                self.rounds.push(DraftRound::default());
            }
            Msg::Delete(index) => {
                self.rounds[index].image_local = None;
                self.rounds[index].image_url = None;
            }
            Msg::Upload(index, file) => {}
            Msg::Clicked(index) => {
                self.current = index;
            }
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let current = self.current;

        let onchange = self.link.callback(move |draft| Msg::Change(current, draft));
        let onremove = self.link.callback(move |_| Msg::Remove);
        let onadd = self.link.callback(|_| Msg::Add);
        let onclick = self.link.callback(Msg::Clicked);
        let ondelete = self.link.callback(move |_| Msg::Delete(current));
        let onupload = self.link.callback(move |files| Msg::Upload(current, files));

        let side_images: Vec<_> = self
            .rounds
            .iter()
            .map(|round| {
                round.image_local.as_ref().map(|x| Url::create_object_url_with_blob(&x).unwrap())
            })
            .collect();

        let side_classes = "is-flex is-flex-direction-column is-justify-content-space-between";
        let draft = self.rounds[current].clone();

        let center = match (&draft.image_local, &draft.image_url) {
            (Some(file), _) => {
                html! { <CenterImage image_url={Url::create_object_url_with_blob(file).unwrap()} /> }
            }
            (_, Some(url)) => html! { <CenterImage image_url={url.clone()} /> },
            _ => html! { {"no image"} },
        };

        html! {
            <Columns>
                <cbs::Sidebar size={ColumnSize::Is2} alignment={cbs::SidebarAlignment::Left} extra={format!("p-0 {}", side_classes)} overflow=false>
                    <SideImages images={side_images} onclick={onclick} current={self.current}/>
                    <div>
                        <hr/>
                        <Buttons extra="mt-auto px-4 py-2">
                            <Button fullwidth=true color={Color::Danger} light=true onclick={onremove}>
                                <span class="icon"> <Icon icon={"fas fa-trash"}/> </span> <span> {"remove round"} </span>
                            </Button>
                            <Button  fullwidth=true color={Color::Success} light=true onclick={onadd}>
                                <span class="icon"> <Icon icon={"fas fa-plus"}/> </span> <span> {"add round"} </span>
                            </Button>
                        </Buttons>
                    </div>
                </cbs::Sidebar>

                <Column size={ColumnSize::Is8}>
                    { center }
                </Column>

                <cbs::Sidebar size={ColumnSize::Is2} alignment={cbs::SidebarAlignment::Right} extra={format!("p-6 {}", side_classes)}>
                    <SideOptions draft={draft} onchange={onchange} ondelete={ondelete} onupload={onupload}/>
                </cbs::Sidebar>
            </Columns>
        }
    }
}
