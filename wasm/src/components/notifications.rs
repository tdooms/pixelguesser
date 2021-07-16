use crate::agents::NotifyAgent;
use crate::notifications::Notification;
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
        let view_entry = |(id, notification): (&i64, &Rc<Notification>)| {
            let id = *id;
            let color = match **notification {
                Notification::Error(_) => "is-danger",
                Notification::Warning(_) => "is-warning",
            };

            html! {
                <div class=classes!("notification", color)>
                    <button onclick=self.link.callback(move |_| Msg::Remove(id)) class="delete"></button>
                    {notification}
                </div>
            }
        };

        html! {
            <div class="m-4" style="position:absolute; z-index:1000">
                { for self.entries.iter().map(view_entry) }
            </div>
        }
    }
}
