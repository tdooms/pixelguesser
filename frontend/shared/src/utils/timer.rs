use gloo::timers::callback::Timeout;
use yew::html::Scope;
use yew::Component;

pub fn set_timer<T: Component, M: 'static>(scope: &Scope<T>, ms: u32, msg: M) -> Timeout
where
    <T as yew::Component>::Message: From<M>,
{
    let cloned = scope.clone();
    Timeout::new(ms, move || cloned.send_message(msg))
}
