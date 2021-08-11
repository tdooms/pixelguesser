use yew::prelude::*;
use yew::utils::NeqAssign;

use crate::classify;

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
            <div class={classes}>
                { for self.props.children.iter() }
            </div>
        }
    }
}
