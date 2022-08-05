use api::{ContentFilter, FilterBy, OrderBy, Orientation, Photo};
use cobul::*;
use components::{DynImage, Height};
use futures::FutureExt;
use shared::use_search;
use std::rc::Rc;
use yew::*;
use ywt::callback;

#[derive(Properties, PartialEq)]
pub struct Props {
    filter: FilterBy,
}

#[function_component(Filter)]
pub fn filter(Props { filter }: &Props) -> Html {
    html! {
        <simple::Tabs<Orientation> fullwidth=true toggle=true value={Orientation::Landscape} onclick={Callback::noop()}/>
    }
}

#[function_component(Unsplash)]
pub fn unsplash() -> Html {
    let func = |filter: FilterBy| api::search_photos(filter).map(|x| x.unwrap());

    let filter = use_state(|| FilterBy::default());
    let photos = use_search((*filter).clone(), func);

    let hovered = use_state_eq(|| None);
    let selected = use_state_eq(|| None);

    let active = use_state(|| false);

    let FilterBy { query, page, per_page, order_by, content_filter, orientation } =
        (*filter).clone();

    let view_photo = |(index, photo): (usize, &Photo)| {
        let onmouseover = callback!(hovered; move |_| hovered.set(Some(index)));
        let onmouseout = callback!(hovered; move |_| hovered.set(None));

        let selected = selected.clone();
        let onclick = callback!(move |_| match Some(index) == *selected {
            true => selected.set(None),
            false => selected.set(Some(index)),
        });

        html! {
            <Column size={ColumnSize::IsOneFifth} class={"has-text-centered"} >
                <div {onmouseover} {onmouseout} {onclick}>
                <DynImage height={Height::Px(100)} src={Rc::new(photo.urls.thumb.clone())} border=true />
                <a href={api::author_link(photo)}> {photo.user.username.clone()} </a>
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

    let trigger = html! { <Button color={Color::Info}><Icon icon={fa::Solid::Filter} /></Button> };
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
        <Help> {"Powered by "} <a href={api::unsplash_link()}> {"Unsplash"} </a> </Help>
        </>
    };

    let body = match photos {
        Some(photos) => html! {
            <>
            <Columns multiline=true> { for photos.iter().enumerate().map(view_photo) } </Columns>
            <Button color={Color::Info}> <span> {"Confirm"} </span> </Button>
            </>
        },
        None => html! {},
    };

    html! {
        <>
        {search}
        {body}
        </>
    }
}
