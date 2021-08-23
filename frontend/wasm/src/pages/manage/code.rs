use yew::prelude::*;
use pbs::prelude::*;
use pbs::properties::Color;

#[derive(Debug, Clone, PartialEq)]
enum State {
    Available,
    Invalid,
    Incorrect,
    None,
}

#[derive(Clone, Properties, PartialEq)]
struct Props {
    state: State,

    onjoin: Callback<()>,
    oncancel: Callback<()>,
    onchange: Callback<String>,
}

#[function_component(Code)]
pub fn code(props: &Props) {
    let (help, help_color, icon_right, input_color) = match props.state {
        State::Available => {
            (Some("This room is available."), Some(Color::Success), Some("fas fa-check"), Some(Color::Success))
        }
        State::Invalid | State::Incorrect => {
            (Some("This room is unavailable."), Some(Color::Danger), Some("fas fa-exclamation-triangle"), Some(Color::Danger))
        }
        State::None => {
            (None, None, None, None)
        }
    };

    html! {
        <Section>
            <Container>
                <cbs::SimpleField label="Session code" help={help} help_color={help_color} icon_right={icon_right}>
                    <Input color={input_color} oninput={props.onchange} />
                </cbs::SimpleField>
                <Buttons>
                    <Button color={Color::Link} onclick={onjoin} disabled={self.state != State::Available}> {"Join"} </Button>
                    <Button color={Color::Link} light=true onclick={oncancel}> {"Cancel"} </Button>
                </Buttons>
            </Container>
        </Section>
    }
}