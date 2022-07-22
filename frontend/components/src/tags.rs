use api::DraftTag;
use cobul::*;
use yew::*;
use ywt::*;

#[derive(Properties, Clone, PartialEq)]
pub struct TagProps {
    pub name: String,
    pub onremove: Callback<()>,
}

#[function_component(TagField)]
pub fn tag_field(props: &TagProps) -> Html {
    html! {
        <Control>
        <Tags addons=true>
            <Tag color={Color::Info}> {props.name.clone()} </Tag>
            <Tag delete=true onclick={props.onremove.clone()} tag="a"/>
        </Tags>
        </Control>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onchange: Callback<Vec<DraftTag>>,

    #[prop_or_default]
    pub placeholder: String,
}

#[function_component(TagsField)]
pub fn tags_field(props: &Props) -> Html {
    let tags = use_state(|| vec![]);
    let current = use_state(|| String::new());
    let onchange = props.onchange.clone();

    let ontag = callback!(current; move |string| {
        current.set(string);
    });
    let onadd = callback!(tags, current, onchange; move |_| {
        let mut new = (*tags).clone();
        new.push(DraftTag{ value: (*current).clone() });
        onchange.emit(new.clone());

        tags.set(new);
        current.set(String::new());
    });
    let onkeypress = callback!(onadd; move |e: KeyboardEvent| {
        (e.key() == "Enter").then(|| onadd.emit(()));
    });
    let onremove = callback!(tags; move |index| {
        let mut new = (*tags).clone();
        new.remove(index);
        onchange.emit(new.clone());
        tags.set(new);
    });

    let view = |(index, tag): (usize, &DraftTag)| {
        let onremove = callback!(onremove; move |_| onremove.emit(index));
        html! { <TagField {onremove} name={tag.value.clone()}/> }
    };

    html! {
        <>
        <simple::Field label="Tags">
            <div {onkeypress}>
            <Input oninput={ontag} value={(*current).clone()} placeholder={props.placeholder.clone()}/>
            </div>
        </simple::Field>

        <Field grouped=true multiline=true>
            { for (*tags).iter().enumerate().map(view) }
        </Field>
        </>
    }
}
