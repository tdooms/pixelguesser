use crate::Page;
use cobul::fa::Solid;
use cobul::*;
use shared::use_form;
use std::rc::Rc;
use yew::*;
use ywt::callback;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub submit: Callback<Rc<api::Credentials>>,
    pub info: AttrValue,
}

#[function_component(Credentials)]
pub fn credentials(Props { submit, info }: &Props) -> Html {
    let credentials = use_state(|| Rc::new(api::Credentials::default()));

    let shown = use_state(|| false);
    let loading = use_state(|| false);

    let input = callback!(credentials; move |new| credentials.set(new));
    let show = callback!(shown; move |_| shown.set(!*shown));

    let form = use_form((*credentials).clone(), input);

    let (kind, icon) = match *shown {
        true => ("text", Solid::EyeSlash),
        false => ("password", Solid::Eye),
    };

    let click = callback!(submit, form, loading; move |_| {
        if form.errors().is_empty() {
            submit.emit((*credentials).clone());
            loading.set(true)
        }
    });

    html! {
        <>
        <Block class="has-text-centered">
            <Title> {"Welcome"} </Title>
            <Block/>
            <p> {info} </p>
        </Block>

        <simple::Field class="mb-0" icon_left={fa::Solid::Envelope} help={form.error("email")}>
            // <Input kind="email" model={form.email()} placeholder="Email Address" />
        </simple::Field>

        <Columns>
            <Column>
                <simple::Field class="mb-0" icon_left={fa::Solid::Envelope} help={form.error("password")}>
                    // <Input kind="password" model={form.password()} placeholder="Email Address" />
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
