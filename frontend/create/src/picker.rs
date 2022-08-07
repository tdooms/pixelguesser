use crate::unsplash::Unsplash;
use api::Image;
use cobul::*;
use strum::IntoEnumIterator;
use yew::html::IntoPropValue;
use yew::*;
use ywt::callback;

#[derive(derive_more::Display, Clone, Copy, Debug, PartialEq, strum::EnumIter)]
pub enum Tab {
    #[display(fmt = "Unsplash")]
    Unsplash,
    #[display(fmt = "Upload")]
    Upload,
    #[display(fmt = "Url")]
    Url,
}

impl Tab {
    pub fn icon(&self) -> String {
        match self {
            Tab::Upload => fa::Solid::Upload.into_prop_value(),
            Tab::Url => fa::Solid::Link.into_prop_value(),
            Tab::Unsplash => fa::Brands::Unsplash.into_prop_value(),
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onchange: Callback<Image>,
}

#[function_component(Picker)]
pub fn picker(props: &Props) -> Html {
    let selected = use_state(|| Tab::Unsplash);
    let url = use_state(|| String::new());
    let hovered = use_state(|| false);

    let onselect = callback!(selected; move |tab| selected.set(tab));
    let onurl = callback!(url; move |string| url.set(string));

    let ondragover = callback!(hovered; move |_| hovered.set(true));
    let ondragleave = callback!(hovered; move |_| hovered.set(false));

    let onchange = props.onchange.clone();
    let onupload = callback!(move |files: Vec<web_sys::File>| {
        ywt::spawn!(onchange; async move {
            let image = Image::from_local(files[0].clone()).await.unwrap();
            onchange.emit(image);
        })
    });

    let ondrop = callback!(onupload; move |ev: DragEvent| {
        ev.prevent_default();

        let files = ev.data_transfer().unwrap().files().unwrap();
        let vec = (0..files.length()).filter_map(|i| files.get(i)).collect();

        onupload.emit(vec);
    });

    let view_tab = |tab| {
        let active = (tab == *selected).then(|| "is-active");
        let onclick = onselect.reform(move |_| tab);

        html! {
            <li class={active} {onclick}>
                <a> <Icon icon={tab.icon()}/> <span> {tab.to_string()} </span> </a>
            </li>
        }
    };

    let body = match *selected {
        Tab::Upload if *hovered => html! {
            <div class="box has-text-centered" style="height:30%" {ondragover} {ondrop} {ondragleave}>
                <Icon icon={fa::Solid::Download} size={Size::Large}/>
            </div>
        },
        Tab::Upload => html! {
            <div class="box has-text-centered" style="height:30%" {ondragover} {ondrop} {ondragleave}>
            <Icon icon={fa::Solid::Upload} size={Size::Large}/>
            <p> {"Select a file or drag here"} </p>
            <Block/>
            <File {onupload} alignment={Alignment::Centered}/>
            </div>
        },
        Tab::Url => html! {
            <>
            <Label> {"Paste your image url here"} </Label>
            <Field grouped=true>
                <Control expanded=true>
                    <Input oninput={onurl} />
                </Control>
                <Control>
                    <Button color={Color::Info}> <Icon icon={fa::Solid::ArrowRight} /> </Button>
                </Control>
            </Field>
            </>
        },
        Tab::Unsplash => html! {
            <Unsplash onselect={props.onchange.clone()}/>
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
