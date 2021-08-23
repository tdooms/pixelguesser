use yew::prelude::*;
use yew::utils::NeqAssign;

use pbs::prelude::*;
use pbs::properties::{Color, ColumnSize};

use crate::components::QuizCard;
use crate::graphql::DraftQuiz;

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
                // TODO: stuff
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
        let filename = image_local.as_ref().map(|x| x.name());

        html! {
            <Section>
                <Container>
                    <Columns>
                        <Column>
                            <cbs::SimpleField label="Name">
                                <Input oninput={self.link.callback(Msg::Name)} />
                            </cbs::SimpleField>

                            <cbs::SimpleField label="Creator">
                                <Input oninput={self.link.callback(Msg::Creator)} />
                            </cbs::SimpleField>

                            <cbs::SimpleField label="Description">
                                <Textarea oninput={self.link.callback(Msg::Description)} />
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
                            <QuizCard name={name.clone()} creator={creator.clone()} description={description.clone()} image_url={image_url.clone()}/>
                        </Column>
                    </Columns>
                </Container>
            </Section>
        }
    }
}
