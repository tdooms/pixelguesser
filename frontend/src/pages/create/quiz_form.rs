use crate::components::QuizCard;
use crate::constants::IMAGE_PLACEHOLDER;
use crate::graphql::DraftQuiz;
use cobul::props::{Color, ColumnSize};
use cobul::*;
use yew::prelude::*;

#[function_component(QuizForm)]
pub fn quiz_form(props: &Form<DraftQuiz>) -> Html {
    const NAME_DEFAULT: &str = "Cities";
    const CREATOR_DEFAULT: &str = "John Doe";
    const DESCRIPTION_DEFAULT: &str = "The best quiz";

    let errors = props.errors();
    let DraftQuiz { name, creator, description, image } = &props.inner;

    html! {
        <>
        <SimpleField label="Quiz Name" help={errors.get("name").cloned()} help_color={Color::Danger}>
            <Input oninput={props.field(|x| &mut x.description)} placeholder={NAME_DEFAULT}/>
        </SimpleField>

        <SimpleField label="Creator" help={errors.get("creator").cloned()} help_color={Color::Danger}>
            <Input oninput={props.field(|x| &mut x.description)} placeholder={CREATOR_DEFAULT}/>
        </SimpleField>

        <SimpleField label="Description" help={errors.get("description").cloned()} help_color={Color::Danger}>
            <Textarea oninput={props.field(|x| &mut x.description)} placeholder={DESCRIPTION_DEFAULT} />
        </SimpleField>

        <SimpleField label="Image" help={errors.get("image").cloned()} help_color={Color::Danger}>
            <File fullwidth=true filename={image.name()} onupload={Callback::noop()}/>
        </SimpleField>

        <Buttons>
            <Button color={Color::Primary} light=true onclick={props.cancel()}> {"cancel"} </Button>
            <Button color={Color::Primary} disabled={!errors.is_empty()} onclick={props.submit()}> {"continue"} </Button>
        </Buttons>
        </>
    }
}
