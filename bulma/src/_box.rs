use yew::prelude::*;

#[derive(Properties, Clone)]
struct Props {
    #[prop_or_default]
    pub children: Children,
}

struct Box {
    props: Props,
}

impl Component for Box {
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
        html! { <div class="box"> { self.props.children } </div> }
    }
}
