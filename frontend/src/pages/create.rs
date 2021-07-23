use std::collections::HashMap;

use yew::prelude::*;
use yew::web_sys::{File as SysFile, Url};

use pbs::{Alignment, Color, ColumnSize, SidebarAlignment};

pub enum Msg {
    Upload(Vec<SysFile>),
    Add,
    Input,
    Remove,
    Reveal,
    Pause,
    Resume,
}

pub struct Create {
    link: ComponentLink<Self>,
    file: Option<SysFile>,
}

impl Component for Create {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, file: None }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Upload(files) => {
                self.file = files.into_iter().next();
                true
            }
            Msg::Input => false,
            Msg::Remove => {
                self.file = None;
                true
            }
            Msg::Reveal => false,
            Msg::Pause => false,
            Msg::Resume => false,
            Msg::Add => false,
        }
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

        let on_upload = self.link.callback(|x| Msg::Upload(x));
        let on_remove = self.link.callback(|_| Msg::Remove);
        let on_add = self.link.callback(|_| Msg::Add);

        let file_or_image = match &self.file {
            Some(file) => html! {
                <>
                <pbs::DynImage src={source.clone()} height=85/>

                <pbs::Buttons alignment={Alignment::Centered} extra="mt-5">
                    <pbs::Button text="reveal" icon="fas fa-eye" onclick={self.link.callback(|_| Msg::Reveal)}/>
                    <pbs::Button text="resume" icon="fas fa-play" onclick={self.link.callback(|_| Msg::Resume)}/>
                    <pbs::Button text="pause" icon="fas fa-pause" onclick={self.link.callback(|_| Msg::Pause)}/>
                </pbs::Buttons>
                </>
            },
            None => html! {
                <pbs::Center>
                    <pbs::File boxed=true alignment={Alignment::Centered} on_upload={on_upload} />
                </pbs::Center>
            },
        };

        let points_values: Vec<(_, _)> = (1..=5).map(|x| (x.to_string(), x)).collect();
        let guesses_values: Vec<(_, _)> = (1..=3).map(|x| (x.to_string(), x)).collect();

        let answer_label = html_nested! { <pbs::Label> {"Answer"} </pbs::Label> };
        let answer_inner = html! { <pbs::Input oninput={self.link.callback(|_| Msg::Input)} /> };

        let points_label = html_nested! { <pbs::Label> {"Points"} </pbs::Label> };
        let points_inner = html! { <pbs::KvButtons<i32> values={points_values} color={Color::Link} alignment={Alignment::Centered} /> };

        let guesses_label = html_nested! { <pbs::Label> {"Guesses"} </pbs::Label> };
        let guesses_inner = html! { <pbs::KvButtons<i32> values={guesses_values} color={Color::Link} alignment={Alignment::Centered} /> };

        let side_images = match &self.file {
            Some(file) => html! { <pbs::DynImage src={source} height=10/> },
            None => html! {},
        };

        html! {
            <pbs::Columns>
                <pbs::Sidebar size=ColumnSize::Is2 alignment={SidebarAlignment::Left}>
                    { side_images }
                </pbs::Sidebar>

                <pbs::Column size=ColumnSize::Is8>
                    { file_or_image }
                </pbs::Column>

                <pbs::Sidebar size=ColumnSize::Is2 alignment={SidebarAlignment::Right}>
                    <pbs::Field label={answer_label}>
                        <pbs::Control inner={answer_inner} />
                    </pbs::Field>
                    <pbs::Field label={points_label}>
                        <pbs::Control inner={points_inner} />
                    </pbs::Field>
                    <pbs::Field label={guesses_label}>
                        <pbs::Control inner={guesses_inner} />
                    </pbs::Field>

                    <pbs::Buttons>
                        <pbs::Button icon="fas fa-trash" fullwidth=true color={Color::Danger} light=true text="remove image" onclick={on_remove}/>
                        <pbs::Button icon="fas fa-plus" fullwidth=true color={Color::Success} light=true text="add round" onclick={on_add}/>
                    </pbs::Buttons>
                </pbs::Sidebar>
            </pbs::Columns>
        }
    }
}
