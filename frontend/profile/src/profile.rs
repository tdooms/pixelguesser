use cobul::*;
use yew::*;

#[function_component(Profile)]
pub fn profile() -> Html {
    html! {
        <Columns centered=true vcentered=true>
            <Column>
                <Box>
                    {"Hello there, profile"}
                </Box>
            </Column>
        </Columns>
    }
}
