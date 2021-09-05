use yew::prelude::*;

use pbs::prelude::*;
use pbs::properties::Alignment;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub onupload: Callback<Vec<web_sys::File>>,
}

#[function_component(SideUpload)]
pub fn side_upload(props: &Props) -> Html {
    html! {
        <cbs::Center>
            <File boxed=true alignment={Alignment::Centered} onupload={props.onupload.clone()} />
        </cbs::Center>
    }
}