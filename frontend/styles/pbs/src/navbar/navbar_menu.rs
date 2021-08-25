use yew::prelude::*;
use yew::utils::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
struct Props {
    children: Children
}

struct NavbarMenu {
    props: Props,
    link: ComponentLink<Self>,
    active: bool
}

impl Component for NavbarMenu {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link, active: false }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        self.active = !self.active;
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let onclick = self.link.callback(|_| ());
        let classes = classes!("navbar-menu", self.active.then(|| "is-active"));

        html! {
            <div class={classes} role="button" aria-label="menu" aria-expanded="false" onclick={onclick}>
                { for props.children.iter() }
            </div>
        }
    }
}