use pbs::{Alignment, Color, ColumnSize};
use std::collections::HashMap;
use yew::prelude::*;
use yew::web_sys::{File as SysFile, Url};

pub enum Msg {
    Upload(Vec<SysFile>),
    Input,
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

        let callback = self.link.callback(|x| Msg::Upload(x));
        let filename = self.file.as_ref().map(|file| file.name());

        let points_values: Vec<(_, _)> = (1..=5).map(|x| (x.to_string(), x)).collect();
        let guesses_values: Vec<(_, _)> = (1..=3).map(|x| (x.to_string(), x)).collect();

        let answer_label = html_nested! { <pbs::Label> {"Answer"} </pbs::Label> };
        let answer_inner = html! { <pbs::Input oninput={self.link.callback(|_| Msg::Input)} /> };

        let points_label = html_nested! { <pbs::Label> {"Points"} </pbs::Label> };
        let points_inner = html! { <pbs::KvButtons<i32> values={points_values} color={Color::Link} alignment={Alignment::Centered} /> };

        let guesses_label = html_nested! { <pbs::Label> {"Guesses"} </pbs::Label> };
        let guesses_inner = html! { <pbs::KvButtons<i32> values={guesses_values} color={Color::Link} alignment={Alignment::Centered} /> };

        html! {
            <pbs::Columns>
                <pbs::Column size=ColumnSize::Is10>
                    <pbs::Section>
                        <pbs::Container>
                            <pbs::Center>
                                <pbs::File boxed=true alignment={Alignment::Centered} on_upload={callback} filename={filename}/>
                                <pbs::DynImage src={source} />
                            </pbs::Center>
                        </pbs::Container>
                    </pbs::Section>
                </pbs::Column>

                <pbs::Sidebar>
                    <pbs::Field label={answer_label}>
                        <pbs::Control inner={answer_inner} />
                    </pbs::Field>
                    <pbs::Field label={points_label}>
                        <pbs::Control inner={points_inner} />
                    </pbs::Field>
                    <pbs::Field label={guesses_label}>
                        <pbs::Control inner={guesses_inner} />
                    </pbs::Field>
                </pbs::Sidebar>
            </pbs::Columns>
        }
    }
}
