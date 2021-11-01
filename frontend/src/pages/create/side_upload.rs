use yew::prelude::*;

use cobul::props::Alignment;
use cobul::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub onupload: Callback<Vec<web_sys::File>>,
}

#[function_component(SideUpload)]
pub fn side_upload(props: &Props) -> Html {
    html! {
        <Center>
            <File boxed=true alignment={Alignment::Centered} onupload={props.onupload.clone()} />
        </Center>
    }
}
