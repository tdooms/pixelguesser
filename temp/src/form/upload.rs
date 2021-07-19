use crate::{Color, Size};
use yew::prelude::*;

pub enum Msg {
    Upload(ChangeData),
}

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub color: Color,

    #[prop_or_default]
    pub filename: Option<String>,

    #[prop_or_default]
    pub accept: Option<String>,

    pub on_upload: Callback<Vec<yew::web_sys::File>>,
}

pub struct Upload {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for Upload {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Upload(ChangeData::Files(files)) => {
                let vec = (0..files.length()).filter_map(|i| files.get(i)).collect();
                self.props.on_upload.emit(vec);
                false
            }
            _ => false, // logic error
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let maybe_file = || match &self.props.filename {
            None => html! {},
            Some(file) => html! {<span class="file-name"> {file} </span>},
        };

        html! {
            <div class=classes!("file", self.props.filename.as_ref().map(|_| "has-name"))>
                <label class="file-label">
                <input class="file-input" type="file" accept={self.props.accept.clone()} onchange=self.link.callback(|x| Msg::Upload(x)) />
                <span class="file-cta">
                    <span class="file-icon">
                    <i class="fas fa-upload"></i>
                    </span>
                    <span class="file-label">
                        {"Choose a file..."}
                    </span>
                </span>
                    { maybe_file() }
                </label>
            </div>
        }
    }
}
