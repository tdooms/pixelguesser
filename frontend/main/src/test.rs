use yew::*;

#[function_component(Test)]
pub fn test() -> Html {
    let style = "animation-name:flash-danger;animation-duration:4s";
    let state = use_state(|| false);

    let cloned = state.clone();
    let onclick = Callback::from(move |_| cloned.set(!*cloned));

    html! {
        <div class="box" style={state.then(|| style)} {onclick}>
        {"the hidden test page"}
        </div>
    }
}
