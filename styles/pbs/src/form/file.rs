use gloo_file::File as SysFile;
use yew::prelude::*;

use crate::{classify, Alignment, Color, Size};

#[derive(Properties, Clone)]
pub struct FileProps {
    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub fullwidth: bool,

    #[prop_or_default]
    pub filename: Option<String>,

    #[prop_or_default]
    pub accept: Option<String>,

    #[prop_or_default]
    pub boxed: bool,

    #[prop_or_default]
    pub alignment: Alignment,

    #[prop_or_default]
    pub extra: String,

    // We don't use gloo_file filelist because it
    // doesn't allow to move from it for some reason.
    pub onupload: Callback<Vec<SysFile>>,
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
                let list =
                    (0..files.length()).filter_map(|i| files.get(i)).map(SysFile::from).collect();

                self.props.onupload.emit(list);
            }
            _ => unreachable!(
                "invariant violation: received non-file change event from a file input element"
            ),
        }
        true
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

        let FileProps { boxed, fullwidth, .. } = self.props;
        let maybe_name = self.props.filename.as_ref().map(|_| "has-name");
        let callback = self.link.callback(|x| x);
        let accept = self.props.accept.clone();

        let classes = classes!(
            "file",
            classify!(fullwidth, boxed),
            maybe_name,
            self.props.alignment.to_string(),
            &self.props.extra
        );

        html! {
            <div class={classes}>
                <label class="file-label">
                <input class="file-input" type="file" accept={accept} onchange={callback} />
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
