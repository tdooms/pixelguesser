use std::collections::HashMap;

use futures::FutureExt;
use yew::prelude::*;

use cobul::props::{Color, ColumnSize};
use cobul::*;

use crate::components::QuizCard;
use crate::constants::IMAGE_PLACEHOLDER;
use crate::graphql::{create_quiz, DraftQuiz, Image};

pub enum Msg {
    Name(String),
    Creator(String),
    Description(String),
    Image(Vec<web_sys::File>),

    QuizUploaded,

    Cancel,
    Continue,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub oncontinue: Callback<DraftQuiz>,
    pub oncancel: Callback<()>,
}

pub struct CreateQuiz {
    draft: DraftQuiz,
    errors: HashMap<String, String>,
}

impl Component for CreateQuiz {
    type Message = Msg;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self { draft: DraftQuiz::default(), errors: HashMap::default() }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Name(name) => self.draft.name = name,
            Msg::Creator(creator) => self.draft.creator = creator,
            Msg::Description(description) => self.draft.description = description,
            Msg::Image(files) if files.len() == 1 => {
                self.draft.image = Image::new(&files[0]);
            }
            Msg::Image(files) => {
                // TODO: give error
            }
            Msg::QuizUploaded => ctx.props().oncontinue.emit(self.draft.clone()),
            Msg::Cancel => {
                ctx.props().oncancel.emit(());
            }
            Msg::Continue => {
                let cloned = self.draft.clone();
                ctx.link().send_future(create_quiz(cloned).map(|_| Msg::QuizUploaded))
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let DraftQuiz { name, creator, description, image } = &self.draft;

        let src = image.src().unwrap_or_else(|| IMAGE_PLACEHOLDER.to_owned());
        let filename = image.name();

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
                            <SimpleField label="Quiz Name" help={self.errors.get("name").cloned()} help_color={Color::Danger}>
                                <Input oninput={ctx.link().callback(Msg::Name)} placeholder={NAME_DEF}/>
                            </SimpleField>

                            <SimpleField label="Creator" help={self.errors.get("creator").cloned()} help_color={Color::Danger}>
                                <Input oninput={ctx.link().callback(Msg::Creator)} placeholder={CREATOR_DEF}/>
                            </SimpleField>

                            <SimpleField label="Description" help={self.errors.get("description").cloned()} help_color={Color::Danger}>
                                <Textarea oninput={ctx.link().callback(Msg::Description)} placeholder={DESCRIPTION_DEF} />
                            </SimpleField>

                            <SimpleField label="Image" help={self.errors.get("image").cloned()} help_color={Color::Danger}>
                                <File fullwidth=true filename={filename} onupload={ctx.link().callback(Msg::Image)}/>
                            </SimpleField>

                            <Buttons>
                                <Button color={Color::Primary} light=true onclick={ctx.link().callback(|_| Msg::Cancel)}> {"cancel"} </Button>
                                <Button color={Color::Primary} disabled={!self.errors.is_empty()} onclick={ctx.link().callback(|_| Msg::Continue)}> {"continue"} </Button>
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
