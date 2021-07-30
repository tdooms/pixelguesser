use pbs::{Color, HeroSize};
use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TitleHeroProps {
    /// Extra classes for the hero container.
    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub color: Option<Color>,
    /// The size for this hero.
    #[prop_or_default]
    pub size: Option<HeroSize>,

    pub title: String,

    pub subtitle: String,
}

pub struct TitleHero {
    props: TitleHeroProps,
}

impl Component for TitleHero {
    type Message = ();
    type Properties = TitleHeroProps;

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
        let classes = classes!(
            "hero",
            self.props.size.as_ref().map(ToString::to_string),
            self.props.color.as_ref().map(ToString::to_string),
            &self.props.extra
        );

        html! {
            <section class={classes}>
                <div class="hero-body">
                    <p class="title"> {&self.props.title} </p>
                    <p class="subtitle"> {&self.props.subtitle} </p>
                </div>
            </section>
        }
    }
}
