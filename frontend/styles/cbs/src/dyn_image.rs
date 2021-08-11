use yew::prelude::*;
use yew::utils::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct DynImageProps {
    #[prop_or_default]
    pub src: String,

    #[prop_or(100)]
    pub height: u32,
}

pub struct DynImage {
    props: DynImageProps,
}

impl Component for DynImage {
    type Message = ();
    type Properties = DynImageProps;

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
        let style = format!("height:{}vh;background-image:url('{}');background-size:contain;background-repeat:no-repeat;background-position:center center;", self.props.height, self.props.src);

        html! { <div style={style}> </div> }
    }
}
