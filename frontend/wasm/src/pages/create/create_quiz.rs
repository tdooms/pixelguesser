use yew::prelude::*;
use yew::utils::NeqAssign;
use web_sys::Url;

use pbs::prelude::*;
use pbs::properties::{Color, ColumnSize};

use crate::components::QuizCard;
use crate::graphql::DraftQuiz;
use crate::constants::PLACEHOLDER;

pub enum Msg {
    Name(String),
    Creator(String),
    Description(String),
    Upload(Vec<web_sys::File>),
    Cancel,
    Continue,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub oncontinue: Callback<DraftQuiz>,
    pub oncancel: Callback<()>,
}

pub struct CreateQuiz {
    link: ComponentLink<Self>,
    props: Props,

    draft: DraftQuiz,
}

impl Component for CreateQuiz {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props, draft: DraftQuiz::default() }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Name(name) => self.draft.name = name,
            Msg::Creator(creator) => self.draft.creator = creator,
            Msg::Description(description) => self.draft.description = description,
            Msg::Upload(files) if files.len() == 1 => {
                self.draft.image_local = Some(files[0].clone());
            }
            Msg::Upload(files) => {
                // TODO: give error
            }
            Msg::Cancel => {
                self.props.oncancel.emit(());
            }
            Msg::Continue => self.props.oncontinue.emit(self.draft.clone()),
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let DraftQuiz { name, creator, description, image_url, image_local } = &self.draft;

        let (src, filename) = match (image_url, image_local) {
            (Some(_), Some(_)) => (PLACEHOLDER.to_owned(), None), // TODO: error
            (Some(url), _) => (url.clone(), Some(url.clone())), // TODO: strip endpoint of first url,
            (_, Some(image)) => (Url::create_object_url_with_blob(image).unwrap(), Some(image.name())),
            (None, None) => (PLACEHOLDER.to_owned(), None)
        };

        let empty_default = |value: String, default: &str| -> String {
            Some(value).filter(|x| !x.is_empty()).unwrap_or_else(|| default.to_owned())
        };

        const NAME_DEF: &str = "Cities";
        const CREATOR_DEF: &str = "John Doe";
        const DESCRIPTION_DEF: &str = "The best quiz";

        let name = empty_default(name.clone(), NAME_DEF);
        let creator = empty_default(creator.clone(), CREATOR_DEF);
        let description = empty_default(description.clone(), DESCRIPTION_DEF);

        html! {
            <Section>
                <Container>
                    <Columns>
                        <Column>
                            <cbs::SimpleField label="Quiz Name">
                                <Input oninput={self.link.callback(Msg::Name)} placeholder={NAME_DEF}/>
                            </cbs::SimpleField>

                            <cbs::SimpleField label="Creator">
                                <Input oninput={self.link.callback(Msg::Creator)} placeholder={CREATOR_DEF}/>
                            </cbs::SimpleField>

                            <cbs::SimpleField label="Description">
                                <Textarea oninput={self.link.callback(Msg::Description)} placeholder={DESCRIPTION_DEF} />
                            </cbs::SimpleField>

                            <cbs::SimpleField label="Image">
                                <File fullwidth=true filename={filename} onupload={self.link.callback(Msg::Upload)}/>
                            </cbs::SimpleField>

                            <Buttons>
                                <Button color={Color::Primary} light=true onclick={self.link.callback(|_| Msg::Cancel)}> {"cancel"} </Button>
                                <Button color={Color::Primary} onclick={self.link.callback(|_| Msg::Continue)}> {"continue"} </Button>
                            </Buttons>

                        </Column>
                        <Column size={ColumnSize::Is4}>
                            <QuizCard name={name} creator={creator.clone()} description={description.clone()} src={src}/>
                        </Column>
                    </Columns>
                </Container>
            </Section>
        }
    }
}
