use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
struct Props {
    children: Children
}

struct NavbarMenu {
    active: bool
}

impl Component for NavbarMenu {
    type Message = ();
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self { active: false }
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        self.active = !self.active;
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| ());
        let classes = classes!("navbar-menu", self.active.then(|| "is-active"));

        html! {
            <div class={classes} role="button" aria-label="menu" aria-expanded="false" onclick={onclick}>
                { for ctx.props().children.iter() }
            </div>
        }
    }
}