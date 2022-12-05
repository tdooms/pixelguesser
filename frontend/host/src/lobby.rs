use std::rc::Rc;

use cobul::*;
use yew::*;

use api::{Participant, Player, Quiz, Session, SELF_ENDPOINT};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub code: String,
    pub session: Rc<Session>,
    pub quiz: Rc<Quiz>,
}

pub fn player_column((name, _): (&String, &Player)) -> Html {
    html! {
        <Column size={ColumnSize::IsNarrow}>
            <Box> <Title size={HeaderSize::Is4}> {name} </Title> </Box>
        </Column>
    }
}

#[function_component(Lobby)]
pub fn lobby(props: &Props) -> Html {
    let Props { session, code, quiz } = &props;
    let url = format!("{}/manage/{}", SELF_ENDPOINT, code);

    // SAFETY: Only errors if the url is too long.
    let generate_qr = || {
        // let buffer = QrCode::new(&url)
        //     .unwrap()
        //     .render::<Rgba<u8>>()
        //     .light_color(Rgba([255, 255, 255, 255]))
        //     .build();

        // let (width, height) = buffer.dimensions();
        // PhotonImage::new(buffer.to_vec(), width, height).get_base64()
        "".to_string()
    };

    let image = use_state(generate_qr);

    let subtitle = match session.participants.contains_key(&Participant::Manager) {
        true => "Quiz master present",
        false => "Scan me to become the quiz master",
    };

    html! {
        <>
        <Hero>
            <Title size={HeaderSize::Is1}> {quiz.title.clone()} </Title>
            <Subtitle size={HeaderSize::Is4}> {quiz.explanation.clone()} </Subtitle>
        </Hero>

        <Hero color={Color::Info} size={HeroSize::Small}>
            <Container class="has-text-centered">
                <Subtitle> {subtitle} </Subtitle>
                <img src={(*image).clone()} />
                <p><a class="title is-3" href={url} target="_blank"> {code} </a></p>
            </Container>
        </Hero>

        <Columns centered={true}>
        <Column size={ColumnSize::Is6}>
            <Columns multiline=true centered=true class="mt-5" >
                { for session.players.iter().map(player_column) }
            </Columns>
        </Column>
        </Columns>
        </>
    }
}
