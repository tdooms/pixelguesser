use std::fmt::Display;
use std::marker::PhantomData;

use yew::prelude::*;

use cobul::props::Color;
use cobul::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct AlertsProps<T: Clone + Display + PartialEq + 'static> {
    pub entries: Vec<T>,
    pub onremove: Callback<usize>,
}

#[derive(Default)]
pub struct Alerts<T: Clone + Display + PartialEq + 'static> {
    phantom: PhantomData<T>,
}

impl<T: Clone + Display + PartialEq + 'static> Component for Alerts<T> {
    type Message = usize;
    type Properties = AlertsProps<T>;

    fn create(_: &Context<Self>) -> Self {
        Self { phantom: PhantomData::default() }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let view_entry = |(id, alert): (usize, &T)| {
            html! {
                <Notification color={Color::Warning} light=true ondelete={ctx.link().callback(move |_| id)}>
                    { format!("{}", alert) }
                </Notification>
            }
        };

        let entries = ctx.props().entries.iter().enumerate().map(view_entry);

        html! {
            <div class="m-4" style="position:absolute; z-index:1000">
                { for entries }
            </div>
        }
    }
}
