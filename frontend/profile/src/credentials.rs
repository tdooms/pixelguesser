use crate::Page;
use cobul::fa::Solid;
use cobul::*;
use std::rc::Rc;
use yew::*;
use ywt::callback;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub submit: Callback<Rc<api::Credentials>>,
    pub page: Callback<Page>,
    pub signup: bool,
}

#[function_component(Credentials)]
pub fn credentials(Props { submit, page, signup }: &Props) -> Html {
    let state = use_state(|| Rc::new(api::Credentials::default()));
    let shown = use_state(|| false);
    let dirty = use_state(|| false);
    let loading = use_state(|| false);

    let onchange = callback!(state; move |new| state.set(new));
    let onshow = callback!(shown; move |_| shown.set(!*shown));

    let form = use_form((*state).clone(), onchange);
    let api::Credentials { email, password } = (**state).clone();

    let (r#type, icon) = match *shown {
        true => (InputType::Text, Solid::EyeSlash),
        false => (InputType::Password, Solid::Eye),
    };

    let onclick = callback!(onsubmit, form, dirty, loading; move |_| {
        if form.errors().is_empty() {
            onsubmit.emit((*state).clone());
            loading.set(true)
        }
        else {
            dirty.set(true);
        }
    });

    let help = |key| {
        let error = form.error(key).filter(|_| *dirty);
        let style = error.as_ref().map(|_| "").unwrap_or("visibility:hidden");
        html! { <Help class="mt-0 mb-2" color={Color::Danger} {style}> { error.unwrap_or(String::from("x")) } </Help> }
    };

    let color = |key| form.error(key).filter(|_| *dirty).map(|_| Color::Danger);

    let onforgot = onpage.reform(|_| Page::Forgot);
    let onsignup = onpage.reform(|_| Page::Signup);
    let onlogin = onpage.reform(|_| Page::Login);

    let forgot = match signup {
        false => {
            html! { <div class="my-2"> <a onclick={onforgot}> {"Forgot your password?"} </a> </div>}
        }
        true => html! {},
    };

    let account = match signup {
        false => {
            html! { <> <span> {"Don't have an account? "} </span><a onclick={onsignup}> {"Sign up"} </a> </> }
        }
        true => {
            html! { <> <span> {"Already have an account? "} </span><a onclick={onlogin}> {"Log in"} </a> </> }
        }
    };

    let info = match signup {
        true => "Sign up to Pixelguesser to continue.",
        false => "Log in to Pixelguesser to continue.",
    };

    html! {
        <Box>
            <Block class="has-text-centered">
                <Title> {"Welcome"} </Title>
                <Block/>
                <p> {info} </p>
            </Block>

            <Field class="mb-0">
                <Control expanded=true left={fa::Solid::Envelope}>
                    <Input r#type={InputType::Email} oninput={form.change(|x| &mut x.email)} value={email} placeholder="Email Address" color={color("email")}/>
                </Control>
            </Field>
            { help("email") }

            <Field grouped=true class="mb-0">
                <Control expanded=true left={fa::Solid::Key}>
                    <Input {r#type} oninput={form.change(|x| &mut x.password)} value={password} placeholder="Password" color={color("password")}/>
                </Control>
                <Control>
                    <Button color={Color::Light}> <Icon {icon} onclick={onshow} /> </Button>
                </Control>
            </Field>
            { help("password") }

            <Button {onclick} loading={*loading} fullwidth=true color={Color::Info}> {"Continue"} </Button>

            {forgot}
            <hr class="mb-2"/>

            <div class="has-text-centered">
                {account}
            </div>
        </Box>
    }
}
