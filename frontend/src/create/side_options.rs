use std::collections::HashMap;

use yew::prelude::*;
use yewtil::NeqAssign;

use api::Round;

pub enum Msg {
    Remove,
    Add,
    Answer(String),
    Points(u32),
    Guesses(u32),
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub onchange: Callback<Round>,
    pub onremove: Callback<()>,
    pub onadd: Callback<()>,
}

pub struct SideImage {
    props: Props,
    answer: Option<String>,
    points: u32,
    guesses: u32,
}

impl Component for SideImage {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props, answer: None, points: 1, guesses: 1 }
    }

    fn update(&mut self, _: Self::Message) -> bool {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let points_values: Vec<(_, _)> = (1..=5).map(|x| (x.to_string(), x)).collect();
        let guesses_values: Vec<(_, _)> = (1..=3).map(|x| (x.to_string(), x)).collect();

        let answer_label = html_nested! { <pbs::Label> {"Answer"} </pbs::Label> };
        let answer_inner = html! { <pbs::Input oninput={self.link.callback(Msg::Answer)} /> };

        let points_label = html_nested! { <pbs::Label> {"Points"} </pbs::Label> };
        let points_inner = html! { <pbs::KvButtons<i32> values={points_values} color={Color::Link} alignment={Alignment::Centered} /> };

        let guesses_label = html_nested! { <pbs::Label> {"Guesses"} </pbs::Label> };
        let guesses_inner = html! { <pbs::KvButtons<i32> values={guesses_values} color={Color::Link} alignment={Alignment::Centered} /> };

        html! {
            <>
            <div>
                <pbs::Field label={answer_label}>
                    <pbs::Control inner={answer_inner} />
                </pbs::Field>
                <pbs::Field label={points_label}>
                    <pbs::Control inner={points_inner} />
                </pbs::Field>
                <pbs::Field label={guesses_label}>
                    <pbs::Control inner={guesses_inner} />
                </pbs::Field>
            </div>

            <pbs::Buttons extra="mt-auto">
                <pbs::Button icon="fas fa-trash" fullwidth=true color={Color::Danger} light=true text="remove image" onclick={onremove}/>
                <pbs::Button icon="fas fa-plus" fullwidth=true color={Color::Success} light=true text="add round" onclick={onadd}/>
            </pbs::Buttons>
            </>
        }
    }
}
