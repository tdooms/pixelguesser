use api::{ContentFilter, FilterBy, Image, OrderBy, Orientation, Photo};
use cobul::*;
use components::{DynImage, Height};
use shared::{use_search, use_toast};
use std::rc::Rc;
use yew::*;
use ywt::callback;

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct Props {
    pub onselect: Callback<Image>,
    pub narrow: bool,
}

#[function_component(Unsplash)]
pub fn unsplash(props: &Props) -> Html {
    let toast = use_toast();
    let func = move |filter: FilterBy| async move {
        toast.maybe(api::search_photos(filter).await).unwrap_or_default()
    };

    let filter = use_state(|| FilterBy::default());

    let cloned = filter.clone();
    use_effect_with_deps(
        move |narrow| {
            match narrow {
                true => cloned.set(FilterBy { per_page: 8, ..(*cloned).clone() }),
                false => cloned.set(FilterBy { per_page: 10, ..(*cloned).clone() }),
            };
            || ()
        },
        props.narrow,
    );

    let photos = use_search((*filter).clone(), func);

    let hovered = use_state_eq(|| None);
    let active = use_state_eq(|| false);

    let FilterBy { order_by, content_filter, orientation, .. } = (*filter).clone();

    let view_photo = |(index, photo): (usize, &Photo)| {
        let onmouseover = callback!(hovered; move |_| hovered.set(Some(index)));
        let onmouseout = callback!(hovered; move |_| hovered.set(None));

        let image = Image::from_unsplash(photo);
        let onclick = props.onselect.reform(move |_| image.clone());

        let class = classes!(
            "has-text-centered",
            "px-0",
            "is-clickable",
            (*hovered == Some(index)).then(|| "has-background-white-ter")
        );

        let (size, height) = match props.narrow {
            true => (ColumnSize::IsOneQuarter, Height::Px(90)),
            false => (ColumnSize::IsOneFifth, Height::Px(100)),
        };

        html! {
            <Column {size} {class} >
                <div {onmouseover} {onmouseout} {onclick}>
                <DynImage {height} src={Rc::new(photo.urls.thumb.clone())} border=true />
                <a href={api::author_link(photo)} target="_blank" onclick={Callback::noop()}> {&photo.user.name} </a>
                </div>
            </Column>
        }
    };

    let oninput = callback!(filter; move |query| {
        filter.set(FilterBy { query, ..(*filter).clone() })
    });

    let onfocus = callback!(active; move |bool| active.set(bool));
    let onorientation = callback!(filter; move |orientation| {
        filter.set(FilterBy { orientation, ..(*filter).clone() })
    });
    let onorderby = callback!(filter; move |order_by| {
        filter.set(FilterBy { order_by, ..(*filter).clone() })
    });
    let oncontentfilter = callback!(filter; move |content_filter| {
        filter.set(FilterBy { content_filter, ..(*filter).clone() })
    });
    let onpage = callback!(filter; move |page| {
        filter.set(FilterBy { page, ..(*filter).clone() })
    });
    let onclick = callback!(active; move |_| active.set(!*active));

    let trigger = html! {
        <Button color={Color::Info} {onclick}>
            <Icon icon={fa::Solid::Filter} />
        </Button>
    };
    let dropdown = html! {
        <Dropdown {trigger} active={*active} right=true {onfocus}>
        <div class="m-3">
        <simple::Field label="Orientation">
            <simple::Tabs<Orientation> fullwidth=true toggle=true value={orientation} onclick={onorientation}/>
        </simple::Field>
        <simple::Field label="Order By">
            <simple::Tabs<OrderBy> fullwidth=true toggle=true value={order_by} onclick={onorderby}/>
        </simple::Field>
        <simple::Field label="Content Filter">
            <simple::Tabs<ContentFilter> fullwidth=true toggle=true value={content_filter} onclick={oncontentfilter}/>
        </simple::Field>
        </div>
        </Dropdown>
    };

    let search = html! {
        <>
        <Label> {"Search for images"} </Label>
        <Field grouped=true>
            <Control expanded=true> <Input {oninput} value={filter.query.clone()}/> </Control>
            <Control> {dropdown} </Control>
        </Field>
        </>
    };

    let body = match photos {
        Some(_) if filter.query.is_empty() => html! {},
        Some(photos) if photos.0.is_empty() => html! {
            "No images found, try broadening your query"
        },
        Some(photos) => html! {
            <>
            <Columns multiline=true> { for photos.0.iter().enumerate().map(view_photo) } </Columns>

            <Columns>
                <Column>
                    <p>{"Powered by "} <a href={api::unsplash_link()} target="_blank"> {"Unsplash"} </a></p>
                </Column>
                <Column size={ColumnSize::IsNarrow}>
                    <simple::Pagination total={photos.1.unwrap()} page={filter.page} onchange={onpage}/>
                </Column>
                <Column/>
            </Columns>
            </>
        },
        None => html! {},
    };

    html! {
        <>
        <Block> {search} </Block>
        {body}
        </>
    }
}
