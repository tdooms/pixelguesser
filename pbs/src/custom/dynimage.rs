use yew::prelude::*;
use yew::web_sys::{HtmlDivElement, HtmlImageElement};
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct DynImageProps {
    #[prop_or_default]
    pub src: String,
}

pub struct DynImage {
    props: DynImageProps,
    container: NodeRef,
    image: NodeRef,
}

impl Component for DynImage {
    type Message = ();
    type Properties = DynImageProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props, container: Default::default(), image: Default::default() }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        // TODO: remove unwraps
        let container = self.container.cast::<HtmlDivElement>().unwrap();
        let image = self.container.cast::<HtmlImageElement>().unwrap();

        image.set_width(container.offset_width() as u32);
        image.set_height(container.offset_height() as u32);

        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        // TODO: trigger update i guess
        html! {
            <div style="height:100vh" ref={self.container.clone()}>
                <img src={self.props.src.clone()} />
            </div>
        }
    }
}
