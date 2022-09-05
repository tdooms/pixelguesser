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
    pub change: Callback<Image>,
    pub narrow: bool,
}

#[function_component(Picker)]
pub fn picker(props: &Props) -> Html {
    let selected = use_state(|| Tab::Unsplash);
    let url = use_state(|| String::new());
    let hovered = use_state(|| false);

    let onselect = callback!(selected; move |tab| selected.set(tab));
    let onurl = callback!(url; move |string| url.set(string));

    let ondragenter = callback!(hovered; move |ev: DragEvent| {
        ev.prevent_default();
        hovered.set(true)
    });
    let ondragover = callback!(move |ev: DragEvent| {
        ev.prevent_default();
    });
    let ondragleave = callback!(hovered; move |ev: DragEvent| {
        ev.prevent_default();
        hovered.set(false)
    });

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

    let ondragend = callback!(|ev: DragEvent| ev.prevent_default());

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
        Tab::Upload => {
            let body = match *hovered {
                true => html! {
                    <Column>
                        <Icon icon={fa::Solid::Download} size={Size::Large}/>
                    </Column>
                },
                false => html! {
                    <Column>
                        <Icon icon={fa::Regular::FileLines} size={Size::Large}/>
                        <p> {"Select a file or drag here"} </p>
                        <Block/>
                        <File {onupload} alignment={Alignment::Centered}/>
                    </Column>
                },
            };
            let class = "columns is-vcentered has-text-centered";
            let style = "height:12rem;border-style:dashed";

            html! {
                <div {class} {style} {ondragover} {ondrop} {ondragleave} {ondragend} {ondragenter}>
                    {body}
                </div>
            }
        }
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
            <Unsplash onselect={props.onchange.clone()} narrow={props.narrow}/>
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
