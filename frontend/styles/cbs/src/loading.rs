use yew::prelude::*;
use yew::utils::NeqAssign;

use pbs::properties::Color;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct LoadingProps {
    #[prop_or_default]
    color: Option<Color>,
}

pub struct Loading {
    props: LoadingProps,
}

impl Component for Loading {
    type Message = ();
    type Properties = LoadingProps;

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
        let classes = classes!("progress", self.props.color.as_ref().map(ToString::to_string));

        html! {
            <section class="section is-fullheight">
                <div class="columns is-centered is-desktop is-vcentered" style="height:100vh">
                    <div class="column is-half">
                        <progress class={classes}></progress>
                    </div>
                </div>
            </section>
        }
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MaybeLoadingProps {
    #[prop_or_default]
    html: Option<Html>,

    #[prop_or_default]
    color: Option<Color>,
}

pub struct MaybeLoading {
    props: MaybeLoadingProps,
}

impl Component for MaybeLoading {
    type Message = ();
    type Properties = MaybeLoadingProps;

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
        match &self.props.html {
            Some(html) => html.clone(),
            None => html! { <Loading color={self.props.color}/> },
        }
    }
}
