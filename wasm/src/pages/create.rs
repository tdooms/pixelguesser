use yew::prelude::*;
use yew::web_sys::{File, Url};

pub enum Msg {
    Upload(Vec<File>),
}

pub struct Create {
    link: ComponentLink<Self>,
    file: Option<File>,
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

        html! {
            <pbs::Box>
                <pbs::Columns>
                    <pbs::Column>
                        <pbs::Upload on_upload=callback filename=filename/>
                    </pbs::Column>
                    <pbs::Column>
                        <img src={source} />
                    </pbs::Column>
                </pbs::Columns>
            </pbs::Box>
        }
    }
}
