use crate::{classify, Help, Icon, Label};
use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct FieldProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub grouped: bool,

    #[prop_or_default]
    pub multiline: bool,

    #[prop_or_default]
    pub addons: bool,

    #[prop_or_default]
    pub right: bool,

    #[prop_or_default]
    pub left: bool,
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
            self.props.left.then(|| "has-icons-left"),
            self.props.right.then(|| "has-icons-right"),
            self.props.multiline.then(|| "is-grouped-multiline"),
            self.props.addons.then(|| "has-addons"),
            self.props.right.then(|| "has-icons-right"),
            self.props.left.then(|| "has-icons-left"),
            classify!(grouped),
        );

        html! {
            <div class=classes>
                { for self.props.children.iter() }
            </div>
        }
    }
}
