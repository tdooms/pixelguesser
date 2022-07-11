use cobul::*;
use qrcode::QrCode;
use std::rc::Rc;
use yew::*;

use api::{FullQuiz, Player, Session, SELF_ENDPOINT};
use image::Rgba;
use photon_rs::PhotonImage;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub code: String,
    pub session: Rc<Session>,
    pub full: Rc<FullQuiz>,
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
    let Props { session, code, full } = &props;
    let url = format!("{}/manage/{}", SELF_ENDPOINT, code);

    // SAFETY: Only errors if the url is too long.
    let generate_qr = || {
        let buffer = QrCode::new(&url)
            .unwrap()
            .render::<Rgba<u8>>()
            .light_color(Rgba([255, 255, 255, 255]))
            .build();

        let (width, height) = buffer.dimensions();
        PhotonImage::new(buffer.to_vec(), width, height).get_base64()
    };

    let image = use_state(generate_qr);

    html! {
        <>
        <Hero>
            <Title size={HeaderSize::Is1}> {full.quiz.title.clone()} </Title>
            <Subtitle size={HeaderSize::Is4}> {full.quiz.explanation.clone()} </Subtitle>
        </Hero>

        <Hero color={Color::Primary} size={HeroSize::Small}>
            <Container class="has-text-centered">
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
