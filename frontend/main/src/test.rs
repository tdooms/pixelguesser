use cobul::use_model;
use derive_more::Display;
use yew::*;

#[derive(Clone, Copy, Debug, PartialEq, Display)]
enum Page {
    #[display(fmt = "Home")]
    Host,
}

#[function_component(Test)]
pub fn test() -> Html {
    let page = use_model(|| Page::Host);

    let body = match page.value {
        Page::Host => html! { <host::Test /> },
    };

    html! {
        // <>
        // <simple::Tabs model={page} />
        // {body}
        // </>

        {body}
    }
}