use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum SidebarAlignment {
    Right,
    Left,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct SidebarProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub extra: String,

    #[prop_or(SidebarAlignment::Right)]
    pub align: SidebarAlignment,
}

pub struct Sidebar {
    props: SidebarProps,
}

impl Component for Sidebar {
    type Message = ();
    type Properties = SidebarProps;

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
        let classes = classes!("p-6", &self.props.extra);

        let style = match self.props.align {
            SidebarAlignment::Right => {
                "box-shadow:-10px 0px 10px 1px #eeeeee;height:100vh;overflow-y:auto"
            }
            SidebarAlignment::Left => {
                "box-shadow:10px 0px -10px 1px #eeeeee;height:100vh;overflow-y:auto"
            }
        };

        html! {
            <div class={classes} style={style}>
                { for self.props.children.iter() }
            </div>
        }
    }
}
