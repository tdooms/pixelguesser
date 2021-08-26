use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub brand: Option<Html>,

    #[prop_or_default]
    pub start: Option<Html>,

    #[prop_or_default]
    pub end: Option<Html>,

    #[prop_or(true)]
    pub burger: bool,
}

pub struct Navbar {
    active: bool,
}

impl Component for Navbar {
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

        let burger = match ctx.props().burger {
            true => html! {
                <a role="button" class={classes} aria-label="menu" aria-expanded="false" data-target="navbar" onclick={onclick}>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                </a>
            },
            false => html! {}
        };

        html! {
            <nav class="navbar" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    { ctx.props().brand.clone().unwrap_or_default() }
                    { burger }
                </div>

                <div id="navbar" class="navbar-menu">
                    <div class="navbar-start">
                        { ctx.props().start.clone().unwrap_or_default() }
                    </div>

                    <div class="navbar-end">
                        { ctx.props().end.clone().unwrap_or_default() }
                    </div>
                </div>
            </nav>
        }
    }
}