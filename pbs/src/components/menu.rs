use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MenuProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,
}

/// A simple menu, for any type of vertical navigation.
///
/// [https://bulma.io/documentation/components/menu/](https://bulma.io/documentation/components/menu/)
pub struct Menu {
    props: MenuProps,
}

impl Component for Menu {
    type Message = ();
    type Properties = MenuProps;

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
        html! {
            <aside class=classes!("menu", &self.props.extra)>
                {self.props.children.clone()}
            </aside>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MenuListProps {
    /// The child `li` elements of this list.
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub extra: String,
}

/// A container for menu list `li` elements.
///
/// [https://bulma.io/documentation/components/menu/](https://bulma.io/documentation/components/menu/)
pub struct MenuList {
    props: MenuListProps,
}

impl Component for MenuList {
    type Message = ();
    type Properties = MenuListProps;

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
        html! {
            <ul class=classes!("menu-list", &self.props.extra)>
                {self.props.children.clone()}
            </ul>
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct MenuLabelProps {
    /// The text of the label.
    #[prop_or_default]
    pub text: String,

    #[prop_or_default]
    pub extra: String,
}

/// A label for a section of the menu.
///
/// [https://bulma.io/documentation/components/menu/](https://bulma.io/documentation/components/menu/)
pub struct MenuLabel {
    props: MenuLabelProps,
}

impl Component for MenuLabel {
    type Message = ();
    type Properties = MenuLabelProps;

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
        html! {
            <p class=classes!("menu-label", &self.props.extra)>
                { self.props.text.clone() }
            </p>
        }
    }
}
