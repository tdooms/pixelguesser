use std::rc::Rc;

use cobul::*;
use shared::callback;
use yew::*;

use api::{ContentFilter, FilterBy, Image, OrderBy, Orientation, Photo};
use components::{DynImage, Height};
use shared::{use_form, use_search, use_toast};

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct Props {
    pub select: Callback<Image>,
    pub narrow: bool,
}

#[function_component(Unsplash)]
pub fn unsplash(props: &Props) -> Html {
    let toast = use_toast();
    let func = move |filter: FilterBy| async move {
        toast.api(api::search_photos(filter).await).unwrap_or_default()
    };

    let filter = use_model(|| Rc::new(FilterBy { per_page: 10, ..Default::default() }));

    let form = use_form(filter.value.clone(), filter.input.clone());
    let photos = use_search((*filter.value).clone(), func);

    let hovered = use_state_eq(|| None);
    let active = use_state_eq(|| false);

    let view_photo = |(index, photo): (usize, &Photo)| {
        let onmouseover = callback!(hovered; move |_| hovered.set(Some(index)));
        let onmouseout = callback!(hovered; move |_| hovered.set(None));

        let image = Image::from_unsplash(photo);
        let onclick = props.select.reform(move |_| image.clone());

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
                <DynImage {height} src={photo.urls.thumb.clone()} border=true />
                <a href={api::author_link(photo)} target="_blank" onclick={Callback::noop()}> {&photo.user.name} </a>
                </div>
            </Column>
        }
    };

    let click = callback!(active; move |_| active.set(!*active));
    let trigger = html! { <simple::Button color={Color::Info} {click} icon={fa::Solid::Filter} /> };

    let dropdown = html! {
        <Dropdown {trigger} active={*active} right=true>
        <div class="m-3">
        <simple::Field label="Orientation">
            <simple::Tabs<Orientation> fullwidth=true toggle=true model={form.orientation()} />
        </simple::Field>
        <simple::Field label="Order By">
            <simple::Tabs<OrderBy> fullwidth=true toggle=true model={form.order_by()} />
        </simple::Field>
        <simple::Field label="Content Filter">
            <simple::Tabs<ContentFilter> fullwidth=true toggle=true model={form.content_filter()} />
        </simple::Field>
        </div>
        </Dropdown>
    };

    let search = html! {
        <>
        <Label> {"Search for images"} </Label>
        <Field grouped=true>
            <Control expanded=true> <Input model={form.query()} /> </Control>
            <Control> {dropdown} </Control>
        </Field>
        </>
    };

    let body = match photos {
        Some(_) if filter.value.query.is_empty() => html! {},
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
                    // <simple::Pagination total={photos.1.unwrap()} model={filter.page()} />
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
