use crate::{Color, Size};
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct FileProps {
    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub filename: Option<String>,

    #[prop_or_default]
    pub accept: Option<String>,

    #[prop_or_default]
    pub extra: String,

    pub on_upload: Callback<Vec<yew::web_sys::File>>,
}

pub struct File {
    link: ComponentLink<Self>,
    props: FileProps,
}

impl Component for File {
    type Message = ChangeData;
    type Properties = FileProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ChangeData::Files(files) => {
                let vec = (0..files.length()).filter_map(|i| files.get(i)).collect();
                self.props.on_upload.emit(vec);
                false
            }
            _ => unreachable!(
                "invariant violation: received non-file change event from a file input element"
            ),
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

        let maybe_name = self.props.filename.as_ref().map(|_| "has-name");
        let callback = self.link.callback(|x| x);
        let accept = self.props.accept.clone();

        html! {
            <div class=classes!("file", maybe_name, &self.props.extra)>
                <label class="file-label">
                <input class="file-input" type="file" accept=accept onchange=callback />
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
