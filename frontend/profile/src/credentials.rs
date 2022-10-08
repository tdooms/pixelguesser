use std::rc::Rc;

use cobul::fa::Solid;
use cobul::*;
use yew::*;

use shared::{callback, use_form};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub submit: Callback<Rc<api::Credentials>>,
    pub info: AttrValue,
}

#[function_component(Credentials)]
pub fn credentials(Props { submit, info }: &Props) -> Html {
    let credentials = use_model(|| Rc::new(api::Credentials::default()));

    let shown = use_state(|| false);
    let loading = use_state(|| false);
    let form = use_form(credentials.value.clone(), credentials.input.clone());

    let show = callback!(shown; move |_| shown.set(!*shown));
    let click = callback!(submit, form, loading; move |_| {
        if !form.errors().is_empty() { return }
        submit.emit(credentials.value.clone());
        loading.set(true)
    });

    let (kind, icon) = match *shown {
        true => ("text", Solid::EyeSlash),
        false => ("password", Solid::Eye),
    };

    html! {
        <>
        <Block class="has-text-centered">
            <Title> {"Welcome"} </Title>
            <Block/>
            <p> {info} </p>
        </Block>

        <simple::Field icon_left={fa::Solid::Envelope} help={form.error("email")}>
            <Input kind="email" model={form.email()} placeholder="Email Address" />
        </simple::Field>

        <Columns>
            <Column class="pr-0">
                <simple::Field class="mb-0" icon_left={fa::Solid::Key} help={form.error("password")}>
                    <Input {kind} model={form.password()} placeholder="Password" />
                </simple::Field>
            </Column>
            <Column size={ColumnSize::IsNarrow}>
                <simple::Button color={Color::Light} click={show} {icon} />
            </Column>
        </Columns>

        <simple::Button {click} loading={*loading} fullwidth=true color={Color::Info} text="Continue" />
        </>
    }
}
