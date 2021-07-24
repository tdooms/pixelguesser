use crate::create::{CenterImage, SideImages, SideOptions};
use api::{DraftRound, Round};
use pbs::{ColumnSize, SidebarAlignment};
use yew::prelude::*;
use yew::web_sys::File as SysFile;

pub enum Msg {
    Upload(usize, SysFile),
    Change(usize, DraftRound),
    Remove(usize),
    Clicked(usize),
    Add,
}

pub struct Create {
    link: ComponentLink<Self>,
    rounds: Vec<DraftRound>,
    current: usize,
}

impl Component for Create {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, current: 0, rounds: vec![DraftRound::default()] }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Upload(_, _) => {}
            Msg::Change(_, _) => {}
            Msg::Remove(_) => {}
            Msg::Add => {}
            Msg::Clicked(_) => {}
        };
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let current = self.current;

        let onchange = self.link.callback(move |draft| Msg::Change(current, draft));
        let onremove = self.link.callback(move |_| Msg::Remove(current));
        let onadd = self.link.callback(|_| Msg::Add);
        let onclick = self.link.callback(Msg::Clicked);

        let image = self.rounds[current].image_url.clone();
        let onupload = self.link.callback(move |file| Msg::Upload(current, file));

        let side_classes = "is-flex is-flex-direction-column is-justify-content-space-between";

        html! {
            <pbs::Columns>
                <pbs::Sidebar size=ColumnSize::Is2 alignment={SidebarAlignment::Left}>
                    <SideImages images=vec![] onclick=onclick />
                </pbs::Sidebar>

                <pbs::Column size=ColumnSize::Is8>
                    <CenterImage onupload={onupload} image={image} />
                </pbs::Column>

                <pbs::Sidebar size=ColumnSize::Is2 alignment={SidebarAlignment::Right} extra={side_classes}>
                    <SideOptions onchange={onchange} onremove={onremove} onadd={onadd} />
                </pbs::Sidebar>
            </pbs::Columns>
        }
    }
}
