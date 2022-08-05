use crate::unsplash::Unsplash;
use api::Image;
use cobul::*;
use strum::IntoEnumIterator;
use yew::html::IntoPropValue;
use yew::*;
use ywt::callback;

#[derive(derive_more::Display, Clone, Copy, Debug, PartialEq, strum::EnumIter)]
pub enum Tab {
    #[display(fmt = "Upload")]
    Upload,
    #[display(fmt = "Url")]
    Url,
    #[display(fmt = "Unsplash")]
    Unsplash,
    #[display(fmt = "Other")]
    Other,
}

impl Tab {
    pub fn icon(&self) -> String {
        match self {
            Tab::Upload => fa::Solid::Upload.into_prop_value(),
            Tab::Url => fa::Solid::Link.into_prop_value(),
            Tab::Unsplash => fa::Brands::Unsplash.into_prop_value(),
            Tab::Other => fa::Solid::Image.into_prop_value(),
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onchange: Callback<Image>,
}

#[function_component(Picker)]
pub fn picker(_props: &Props) -> Html {
    let selected = use_state(|| Tab::Upload);

    let onclick = callback!(selected; move |tab| selected.set(tab));

    let view_tab = |tab| {
        let active = (tab == *selected).then(|| "is-active");
        let onclick = onclick.reform(move |_| tab);

        html! { <li class={active} {onclick}> <a> <Icon icon={tab.icon()}/> <span> {tab.to_string()} </span> </a> </li> }
    };

    let body = match *selected {
        Tab::Upload => html! {
            <Box style="height: 20%"/>
        },
        Tab::Url => html! {
            "tesst"
        },
        Tab::Unsplash => html! {
            <Unsplash />
        },
        Tab::Other => html! {
            "Other stuffz"
        },
    };

    html! {
        <Column>
        <div style="height:20%"/>
        <Box>
        <Tabs alignment={Alignment::Centered} boxed=true> { for Tab::iter().map(view_tab) }</Tabs>
        {body}
        </Box>
        </Column>
    }
}
