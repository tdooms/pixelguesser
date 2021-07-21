use crate::Color;
use std::collections::HashMap;
use std::fmt::Display;
use yew::prelude::*;
use yewtil::NeqAssign;

pub struct AlertsProps<T: Display> {
    entries: Vec<T>,
}

pub struct Alerts<T: Display> {
    link: ComponentLink<Self>,
    props: AlertsProps<T>,
    counter: u64,
}

impl<T: Display> Component for Alerts<T> {
    type Message = u64;
    type Properties = AlertsProps<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            counter: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        self.entries.remove(&msg).is_some()
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let view_entry = |(id, alert): (usize, &T)| {
            html! {
                <crate::Notification color=Color::Warning light=true onclick=self.link.callback(move |_| id)>
                    { format!("{}", alert) }
                </crate::Notification>
            }
        };

        let entries = self.props.entries.iter().enumerate().map(view_entry);

        html! {
            <div class="m-4" style="position:absolute; z-index:1000">
                { for entries }
            </div>
        }
    }
}
