use crate::agents::NotifyAgent;
use crate::notifications::Notification;
use pbs::Color;
use std::collections::HashMap;
use std::rc::Rc;
use yew::prelude::*;

pub enum Msg {
    New(Rc<Notification>),
    Remove(i64),
}

pub struct Notifications {
    link: ComponentLink<Self>,
    _log_agent: Box<dyn Bridge<NotifyAgent>>,
    counter: i64,
    entries: HashMap<i64, Rc<Notification>>,
}

impl Component for Notifications {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            _log_agent: NotifyAgent::bridge(link.callback(Msg::New)),
            link,
            counter: 0,
            entries: HashMap::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Remove(id) => {
                self.entries.remove(&id);
            }
            Msg::New(entry) => {
                self.entries.insert(self.counter, entry);
                self.counter += 1;
            }
        }

        true
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let color_of = |notification: &Notification| match notification {
            Notification::Error(_) => Color::Danger,
            Notification::Warning(_) => Color::Warning,
        };

        let view_entry = |id: i64, notification: &Notification| {
            html! {
                <pbs::Notification color={color_of(&notification)} onclick=self.link.callback(move |_| Msg::Remove(id))>
                    { notification }
                </pbs::Notification>
            }
        };

        let entries = self
            .entries
            .iter()
            .map(|(id, notification)| view_entry(*id, &notification));

        html! {
            <div class="m-4" style="position:absolute; z-index:1000">
                { for entries }
            </div>
        }
    }
}
