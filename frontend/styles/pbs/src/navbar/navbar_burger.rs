use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
struct Props {
    children: Children
}

struct NavbarBurger {
    active: bool
}

impl Component for NavbarBurger {
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
        let classes = classes!("navbar-burger", self.active.then(|| "is-active"));

        html! {
            <a class={classes} role="button" aria-label="menu" aria-expanded="false" onclick={onclick}>
                <span aria-hidden="true"></span>
                <span aria-hidden="true"></span>
                <span aria-hidden="true"></span>
            </a>
        }
    }
}