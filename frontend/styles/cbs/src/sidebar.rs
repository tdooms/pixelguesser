use yew::prelude::*;

use pbs::properties::ColumnSize;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum SidebarAlignment {
    Right,
    Left,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub extra: String,

    #[prop_or(SidebarAlignment::Right)]
    pub alignment: SidebarAlignment,

    #[prop_or(ColumnSize::Is2)]
    pub size: ColumnSize,

    #[prop_or(true)]
    pub overflow: bool,

    #[prop_or_default]
    pub footer: Option<Html>,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &Props) -> Html {
    let footer_class = props.footer.as_ref().map(|_| "is-flex is-flex-direction-column is-justify-content-space-between");

    let classes = classes!("column", &props.extra, props.size, footer_class);

    let shadow = match props.alignment {
        SidebarAlignment::Right => "-10px 0px 10px 1px #eeeeee",
        SidebarAlignment::Left => "10px 0px 10px 1px #eeeeee",
    };

    let overflow = props.overflow.then(|| "overflow-y:auto").unwrap_or_default();
    let style = format!("box-shadow:{};height:100vh;{}", shadow, overflow);

    let inner = match &props.footer {
        Some(html) => html! {
            <>
                { for props.children.iter() }
                <hr />
                { html.clone() }
            </>
        },
        None => html! { { for props.children.iter() } }
    };

    html! {
        <div class={classes} style={style}>
            { inner }
        </div>
    }
}
