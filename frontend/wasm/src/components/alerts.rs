use std::fmt::Display;

use yew::prelude::*;

use pbs::Color;

#[derive(Clone, Debug, Properties)]
pub struct AlertsProps<T: Clone + Display + 'static> {
    pub entries: Vec<T>,
}

pub struct Alerts<T: Clone + Display + 'static> {
    link: ComponentLink<Self>,
    props: AlertsProps<T>,
}

impl<T: Clone + Display + 'static> Component for Alerts<T> {
    type Message = usize;
    type Properties = AlertsProps<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        // TODO: technically not panic safe...
        self.props.entries.remove(msg);
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let view_entry = |(id, alert): (usize, &T)| {
            html! {
                <pbs::Notification color={Color::Warning} light=true onclick={self.link.callback(move |_| id)}>
                    { format!("{}", alert) }
                </pbs::Notification>
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
