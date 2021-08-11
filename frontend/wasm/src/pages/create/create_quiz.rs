use validator::ValidationErrors;
use yew::prelude::*;

use api::NewQuiz;
use gloo_file::{File, FileReadError};
use gloo_file::callbacks::{FileReader, read_as_bytes};
use pbs::{Color, ColumnSize};
use yew::utils::NeqAssign;

use crate::components::QuizCard;
use crate::utils::bytes_to_url;

pub enum Msg {
    Name(String),
    Creator(String),
    Description(String),
    Upload(Vec<File>),
    Cancel,
    Continue,
    Read(Result<Vec<u8>, FileReadError>),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub oncontinue: Callback<NewQuiz>,
    pub oncancel: Callback<()>,
}

pub struct CreateQuiz {
    link: ComponentLink<Self>,
    props: Props,
    reader: Option<FileReader>,

    draft: NewQuiz,
    image_name: Option<String>,
    errors: ValidationErrors,
}

impl Component for CreateQuiz {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            reader: None,
            draft: DraftQuiz::default(),
            errors: ValidationErrors::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Name(name) => self.draft.name = name,
            Msg::Creator(creator) => self.draft.creator = creator,
            Msg::Description(description) => self.draft.description = description,
            Msg::Upload(mut files) if files.len() == 1 => {
                self.draft.image_name = Some(files[0].name());

                let scope = self.link.clone();
                let callback = move |x| scope.send_message(Msg::Read(x));

                self.reader = Some(read_as_bytes(&files[0], callback))
            }
            Msg::Upload(_) => {
                // give error
            }
            Msg::Cancel => {
                self.props.oncancel.emit(());
            }
            Msg::Continue => {}
            Msg::Read(Ok(bytes)) => {
                self.draft.image_bytes = Some(bytes);
            }
            Msg::Read(Err(err)) => {
                // give error
            }
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let DraftQuiz { name, creator, description, image_name, .. } = &self.draft;
        let image_url = self.draft.image_bytes.as_ref().map(bytes_to_url);

        html! {
            <pbs::Section>
                <pbs::Container>
                    <pbs::Columns>
                        <pbs::Column>
                            <cbs::SimpleField label="Name">
                                <pbs::Input oninput={self.link.callback(Msg::Name)} />
                            </cbs::SimpleField>

                            <cbs::SimpleField label="Creator">
                                <pbs::Input oninput={self.link.callback(Msg::Creator)} />
                            </cbs::SimpleField>

                            <cbs::SimpleField label="Description">
                                <pbs::TextArea oninput={self.link.callback(Msg::Description)} />
                            </cbs::SimpleField>

                            <cbs::SimpleField label="Image">
                                <pbs::File fullwidth=true filename={image_name.clone()} onupload={self.link.callback(Msg::Upload)}/>
                            </cbs::SimpleField>

                            <pbs::Buttons>
                                <cbs::IconButton text="cancel" color={Color::Primary} light=true onclick={self.link.callback(|_| Msg::Cancel)}/>
                                <cbs::IconButton text="continue" color={Color::Primary} onclick={self.link.callback(|_| Msg::Continue)}/>
                            </pbs::Buttons>

                        </pbs::Column>
                        <pbs::Column size={ColumnSize::Is4}>
                            <QuizCard name={name.clone()} creator={creator.clone()} description={description.clone()} image_url={image_url}/>
                        </pbs::Column>
                    </pbs::Columns>
                </pbs::Container>
            </pbs::Section>
        }
    }
}
