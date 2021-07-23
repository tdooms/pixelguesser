use std::collections::HashMap;

use yew::prelude::*;
use yew::web_sys::{File as SysFile, Url};

use api::Round;
use pbs::{Alignment, Color, ColumnSize, SidebarAlignment};

pub enum Msg {
    Upload(usize, SysFile),
    Change(usize, Round),
    Remove(usize),
    Clicked(usize),
    Add,
}

pub struct Create {
    link: ComponentLink<Self>,
    rounds: Vec<Option<Round>>,
    current: Option<usize>,
}

impl Component for Create {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, current: None, rounds: vec![] }
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
        let source = self
            .file
            .as_ref()
            .map(|file| Url::create_object_url_with_blob(file).unwrap())
            .unwrap_or_default();

        let on_upload = ;
        let on_remove = self.link.callback(|_| Msg::Remove);
        let on_add = self.link.callback(|_| Msg::Add);

        let side_classes = "is-flex is-flex-direction-column is-justify-content-space-between";

        let round = self.current.map(|index| self.rounds[index]).flatten();

        match self.current {
            Some(current) => {

            }
            None => html! {
                "please select a round"
            }
        }
        let onupload = self.link.callback(|x| Msg::Upload(x));

        html! {
            <pbs::Columns>
                <pbs::Sidebar size=ColumnSize::Is2 alignment={SidebarAlignment::Left}>
                    { side_images }
                </pbs::Sidebar>

                <pbs::Column size=ColumnSize::Is8>
                    <CenterImage onupload={}>
                </pbs::Column>

                <pbs::Sidebar size=ColumnSize::Is2 alignment={SidebarAlignment::Right} extra={side_classes}>
                    <SideOptions >
                </pbs::Sidebar>
            </pbs::Columns>
        }
    }
}
