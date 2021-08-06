use yew::prelude::*;
use yewtil::NeqAssign;

use crate::classify;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ModalCardProps {
    #[prop_or_default]
    pub children: Children,

    pub title: String,

    #[prop_or_default]
    pub footer: Option<Html>,

    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub active: bool,
}

pub struct ModalCard {
    props: ModalCardProps,
}

impl Component for ModalCard {
    type Message = ();
    type Properties = ModalCardProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let active = self.props.active;
        let classes = classes!("modal", &self.props.extra, classify!(active));

        let footer = match &self.props.footer {
            Some(html) => html.clone(),
            None => html! {},
        };

        html! {
            <div class={classes}>
                <div class="modal-background"></div>
                <div class="modal-card">
                    <header class="modal-card-head">
                        <p class="modal-card-title">{ self.props.title.clone() }</p>
                        <button class="delete" aria-label="close"></button>
                    </header>
                    <section class="modal-card-body">
                        { for self.props.children.iter() }
                    </section>
                    <footer class="modal-card-foot">
                        { footer }
                    </footer>
                </div>
            </div>
        }
    }
}
