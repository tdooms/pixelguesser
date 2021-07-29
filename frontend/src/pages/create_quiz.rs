use std::str::FromStr;

use api::Quiz;
use chrono::{Date, DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use pbs::ColumnSize;
use yew::prelude::*;

use crate::components::quiz_card;

pub enum Msg {
    Name(String),
    Creator(String),
    Description(String),
    Upload(String),
}

#[derive(Properties, Clone)]
pub struct Props {}

pub struct CreateQuiz {
    link: ComponentLink<Self>,
    quiz: Quiz,
}

impl Component for CreateQuiz {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let quiz = Quiz {
            quiz_id: 0,
            name: "".to_string(),
            creator: "".to_string(),
            description: "".to_string(),
            image_url: "".to_string(),
            time_created: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc),
        };

        Self { link, quiz }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Name(name) => self.quiz.name = name,
            Msg::Creator(creator) => self.quiz.creator = creator,
            Msg::Description(description) => self.quiz.description = description,
            Msg::Upload(image_url) => self.quiz.image_url = image_url,
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
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
                            <pbs::Field label=html_nested!{<pbs::Label>{"description"} </pbs::Label>}>
                                <pbs::Control inner={html!{ <pbs::TextArea oninput={self.link.callback(Msg::Description)} />}} />
                            </pbs::Field>
                            <pbs::File onupload={self.link.callback(|_| Msg::Upload("banana.jpg".to_owned()))}/>
                        </pbs::Column>
                        <pbs::Column size={ColumnSize::Is4}>
                            { quiz_card(&self.quiz) }
                        </pbs::Column>
                    </pbs::Columns>
                </pbs::Container>
            </pbs::Section>
        }
    }
}
