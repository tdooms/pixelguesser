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

    #[prop_or_default]
    pub placeholder: String,
}

#[function_component(TagsField)]
pub fn tags_field(props: &Props) -> Html {
    let tags = use_state(|| vec![]);
    let current = use_state(|| String::new());

    let input = props.input.clone();

    let change = callback!(current; move |string| {
        current.set(string);
    });
    let add = callback!(tags, current, input; move |_| {
        let mut new = (*tags).clone();
        new.push(Tag{ value: (*current).clone(), quiz_id: None });
        input.emit(new.clone());

        tags.set(new);
        current.set(String::new());
    });
    let keypress = callback!(add; move |e: KeyboardEvent| {
        (e.key() == "Enter").then(|| add.emit(()));
    });
    let remove = callback!(tags; move |index| {
        let mut new = (*tags).clone();
        new.remove(index);
        input.emit(new.clone());
        tags.set(new);
    });

    let view = |(index, tag): (usize, &Tag)| {
        let remove = callback!(remove; move |_| remove.emit(index));
        html! { <TagField {remove} name={tag.value.clone()}/> }
    };

    html! {
        <>
        <simple::Field label="Tags">
            <div onkeypress={keypress}>
            <Input input={change} value={(*current).clone()} placeholder={props.placeholder.clone()}/>
            </div>
        </simple::Field>

        <Field grouped=true multiline=true>
            { for (*tags).iter().enumerate().map(view) }
        </Field>
        </>
    }
}
