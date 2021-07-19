use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub vcentered: bool,

    #[prop_or_default]
    pub multiline: bool,

    #[prop_or_default]
    pub mobile: bool,
}

pub struct Columns {
    props: Props,
}

impl Component for Columns {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! { <div class="columns"> { for self.props.children.iter() } </div> }
    }
}
