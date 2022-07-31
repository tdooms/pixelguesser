use api::{Image, Photo, UNSPLASH_KEY};
use cobul::*;
use futures::future::FutureExt;
use shared::use_search;
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
pub fn picker(props: &Props) -> Html {
    let query = use_state(|| String::new());
    let oninput = callback!(query; move |string| query.set(string));

    let func =
        |query: String| api::search_photos(query, UNSPLASH_KEY.to_owned()).map(|x| x.unwrap());
    let photos = use_search((*query).clone(), func);

    let selected = use_state(|| Tab::Upload);
    let onclick = callback!(selected; move |tab| selected.set(tab));

    let view_tab = |tab| {
        let active = (tab == *selected).then(|| "is-active");
        let onclick = onclick.reform(move |_| tab);

        html! { <li class={active} {onclick}> <a> <Icon icon={tab.icon()}/> <span> {tab.to_string()} </span> </a> </li> }
    };

    let view_photo = |photo: &Photo| {
        html! { photo.user.first_name.clone() }
    };

    let body = match (*selected, photos.clone()) {
        (Tab::Upload, _) => html! {
            <Box style="height: 20%"/>
        },
        (Tab::Url, _) => html! {
            "tesst"
        },
        (Tab::Unsplash, Some(photos)) => html! {
            <>
            <Input {oninput} value={(*query).clone()}/>
            { for photos.iter().map(view_photo) }
            </>
        },
        (Tab::Unsplash, None) => html! {
            "loading"
        },
        (Tab::Other, _) => html! {
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
