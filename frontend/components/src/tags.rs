use api::Tag;
use cobul::*;
use yew::*;
use ywt::*;

#[derive(Properties, Clone, PartialEq)]
pub struct TagProps {
    pub name: String,
    pub remove: Callback<()>,
}

#[function_component(TagField)]
pub fn tag_field(props: &TagProps) -> Html {
    html! {
        <Control>
        <Tags addons=true>
            <cobul::Tag color={Color::Info}> {props.name.clone()} </cobul::Tag>
            <cobul::Tag delete=true click={props.remove.clone()} tag="a"/>
        </Tags>
        </Control>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub input: Callback<Vec<Tag>>,

    pub value: Vec<Tag>,

    #[prop_or_default]
    pub model: Option<Model<Vec<Tag>>>,

    #[prop_or_default]
    pub placeholder: Option<AttrValue>,
}

#[function_component(TagsField)]
pub fn tags_field(props: &Props) -> Html {
    let current = use_state(|| String::new());

    let (callback, tags) = match props.model.clone() {
        Some(Model { input, value }) => (input, value),
        None => (props.input.clone(), props.value.clone()),
    };

    let input = callback!(current; move |string| {
        current.set(string);
    });
    let add = callback!(tags, current, callback; move |_| {
        let mut new = tags.clone();
        new.push(Tag{ value: (*current).clone(), quiz_id: None });

        callback.emit(new.clone());
        current.set(String::new());
    });
    let remove = callback!(tags; move |index| {
        let mut new = tags.clone();
        new.remove(index);

        callback.emit(new.clone());
    });
    let keypress = callback!(add; move |e: KeyboardEvent| {
        (e.key() == "Enter").then(|| add.emit(()));
    });

    let view = |(index, tag): (usize, &Tag)| {
        let remove = callback!(remove; move |_| remove.emit(index));
        html! { <TagField {remove} name={tag.value.clone()}/> }
    };

    html! {
        <>
        <simple::Field label="Tags">
            <div onkeypress={keypress}>
            <Input {input} value={(*current).clone()} placeholder={props.placeholder.clone()}/>
            </div>
        </simple::Field>

        <Field grouped=true multiline=true>
            { for (*tags).iter().enumerate().map(view) }
        </Field>
        </>
    }
}
