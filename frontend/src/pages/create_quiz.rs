use yew::prelude::*;
use yew::web_sys::File as SysFile;
use yew::web_sys::Url;

use pbs::*;

use crate::components::QuizCard;
use crate::route::Route;

pub enum Msg {
    Name(String),
    Creator(String),
    Description(String),
    Upload(Vec<SysFile>),
    Cancel,
    Continue,
}

#[derive(Properties, Clone)]
pub struct Props {}

pub struct CreateQuiz {
    link: ComponentLink<Self>,
    name: String,
    creator: String,
    description: String,
    image: Option<SysFile>,
}

impl Component for CreateQuiz {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            name: String::new(),
            creator: String::new(),
            description: String::new(),
            image: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Name(name) => self.name = name,
            Msg::Creator(creator) => self.creator = creator,
            Msg::Description(description) => self.description = description,
            Msg::Upload(files) => self.image = files.iter().cloned().next(),
            Msg::Cancel => {
                yew_router::push_route(Route::Overview);
            }
            Msg::Continue => {}
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let filename = self.image.clone().map(|file| file.name()).unwrap_or_default();
        let image = self.image.as_ref().map(|file| Url::create_object_url_with_blob(file).unwrap());

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
                                <pbs::File fullwidth=true filename={filename} onupload={self.link.callback(Msg::Upload)}/>
                            </cbs::SimpleField>

                            <pbs::Buttons>
                                <cbs::IconButton text="cancel" color={Color::Primary} light=true onclick={self.link.callback(|_| Msg::Cancel)}/>
                                <cbs::IconButton text="continue" color={Color::Primary} onclick={self.link.callback(|_| Msg::Continue)}/>
                            </pbs::Buttons>

                        </pbs::Column>
                        <pbs::Column size={ColumnSize::Is4}>
                            <QuizCard name={self.name.clone()} creator={self.creator.clone()} description={self.description.clone()} image_url={image}/>
                        </pbs::Column>
                    </pbs::Columns>
                </pbs::Container>
            </pbs::Section>
        }
    }
}
