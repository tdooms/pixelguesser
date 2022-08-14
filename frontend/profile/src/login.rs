use cobul::*;
use yew::*;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <Columns centered=true vcentered=true>
            <Column>
                <Box>
                    {"Hello there, login"}
                </Box>
            </Column>
        </Columns>
    }
}
