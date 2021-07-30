use std::str::FromStr;

use chrono::{Date, DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use yew::prelude::*;

use api::Quiz;
use pbs::*;

use crate::components::QuizCard;
use crate::route::Route;

pub enum Msg {
    Name(String),
    Creator(String),
    Description(String),
    Upload(String),
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
    image_url: Option<String>,
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
            image_url: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Name(name) => self.name = name,
            Msg::Creator(creator) => self.creator = creator,
            Msg::Description(description) => self.description = description,
            Msg::Upload(image_url) => self.image_url = Some(image_url),
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
        let filename = self.image_url.clone().unwrap_or_default();

        html! {
            <pbs::Section>
                <pbs::Container>
                    <pbs::Columns>
                        <pbs::Column>
                            <pbs::Field label=html_nested!{<pbs::Label>{"name"} </pbs::Label>}>
                                <pbs::Control inner={html!{ <pbs::Input oninput={self.link.callback(Msg::Name)} />}} />
                            </pbs::Field>
                            <pbs::Field label=html_nested!{<pbs::Label>{"creator"} </pbs::Label>}>
                                <pbs::Control inner={html!{ <pbs::Input oninput={self.link.callback(Msg::Creator)} />}} />
                            </pbs::Field>
                            <pbs::Field>
                                <pbs::Label>{"description"} </pbs::Label>
                                <pbs::Control>
                                    <pbs::TextArea oninput={self.link.callback(Msg::Description)} />
                                </pbs::Control>
                            </pbs::Field>

                            <pbs::File fullwidth=true filename={filename} onupload={self.link.callback(|_| Msg::Upload("banana.jpg".to_owned()))}/>

                            <pbs::Buttons alignment={Alignment::Right}>
                                <pbs::Button text="cancel" color={Color::Primary} light=true onclick={self.link.callback(|_| Msg::Cancel)}/>
                                <pbs::Button text="continue" color={Color::Primary} onclick={self.link.callback(|_| Msg::Continue)}/>
                            </pbs::Buttons>
                        </pbs::Column>
                        <pbs::Column size={ColumnSize::Is4}>
                            <QuizCard name={self.name.clone()} creator={self.creator.clone()} description={self.description.clone()} />
                        </pbs::Column>
                    </pbs::Columns>
                </pbs::Container>
            </pbs::Section>
        }
    }
}
