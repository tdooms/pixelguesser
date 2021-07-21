use crate::{classify, Control, Help, Label};
use yew::prelude::*;
use yew::virtual_dom::VChild;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct FieldProps {
    #[prop_or_default]
    pub label: Option<VChild<Label>>,

    pub children: ChildrenWithProps<Control>,

    #[prop_or_default]
    pub help: Option<VChild<Help>>,

    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub grouped: bool,

    #[prop_or_default]
    pub multiline: bool,

    #[prop_or_default]
    pub addons: bool,
}

pub struct Field {
    props: FieldProps,
}

impl Component for Field {
    type Message = ();
    type Properties = FieldProps;

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
        let FieldProps { grouped, .. } = self.props;

        let classes = classes!(
            "field",
            &self.props.extra,
            self.props.multiline.then(|| "is-grouped-multiline"),
            self.props.addons.then(|| "has-addons"),
            classify!(grouped),
        );

        html! {
            <div class=classes>
                { self.props.label.clone().map(Html::from).unwrap_or(html!{}) }
                { for self.props.children.iter() }
                { self.props.help.clone().map(Html::from).unwrap_or(html!{}) }
            </div>
        }
    }
}
