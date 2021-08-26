use yew::prelude::*;
use crate::properties::Hoverable;


// TODO: custom trigger without font awesome
#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub hoverable: Hoverable,

    pub text: String
}

struct Dropdown {
    active: bool
}

impl Component for Dropdown {
    type Message = ();
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self { active: false}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        self.active = !self.active;
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let classes = classes!("dropdown", &props.extra, props.hoverable, ctx.active.then(|| "is-active"));

        html! {
            <div class={classes}>
                <div class="dropdown-trigger">
                    <button class="button" aria-haspopup="true" aria-controls="dropdown-menu3">
                        <span> {ctx.props().text.clone()} </span>
                        <span class="icon is-small"> <i class="fas fa-angle-down" aria-hidden="true"></i> </span>
                    </button>
                </div>
                <div class="dropdown-menu" role="menu">
                    <div class="dropdown-content">
                        { for props.children.iter() }
                    </div>
                </div>
            </div>
        }
    }
}