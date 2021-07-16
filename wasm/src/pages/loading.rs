use yew::prelude::*;

pub fn loading() -> Html {
    html! {
        <section class="section is-fullheight">
            <div class="columns is-centered is-desktop is-vcentered" style="height:100vh">
                <div class="column is-half">
                    <progress class="progress is-primary"></progress>
                </div>
            </div>
        </section>
    }
}

pub fn view_or_loading<T>(option: Option<T>, view: impl FnOnce(T) -> Html) -> Html {
    match option {
        Some(t) => view(t),
        None => html! { loading() },
    }
}
