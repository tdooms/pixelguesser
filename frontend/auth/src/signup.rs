use cobul::*;
use yew::*;

#[function_component(Signup)]
pub fn signup() -> Html {
    html! {
        <Columns centered=true vcentered=true>
            <Column>
                <Box>
                    {"Hello there, signup"}
                </Box>
            </Column>
        </Columns>
    }
}
