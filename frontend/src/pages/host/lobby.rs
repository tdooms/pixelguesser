use cobul::props::{Color, ColumnSize, HeroSize};
use cobul::*;
use qrcode::QrCode;
use sessions::Session;
use yew::prelude::*;

use crate::graphql::Quiz;
use crate::shared::SELF_ENDPOINT;
use image::Rgba;
use photon_rs::PhotonImage;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub session: Session,
    pub code: String,
    pub quiz: Quiz,
}

#[function_component(Lobby)]
pub fn lobby(props: &Props) -> Html {
    let Props { session, code, quiz } = &props;

    let generate = || {
        log::error!("generating QR-code");
        let url = format!("{}/manage/{}", SELF_ENDPOINT, code);

        let buffer = QrCode::new(url)
            .unwrap()
            .render::<Rgba<u8>>()
            .light_color(Rgba([255, 255, 255, 255]))
            .build();

        let (width, height) = buffer.dimensions();
        PhotonImage::new(buffer.to_vec(), width, height).get_base64()
    };

    let image = use_state(generate);

    let players = session.players.iter().map(|(name, _)| {
        html! { <Column size={ColumnSize::IsNarrow}> <Box> {name} </Box> </Column> }
    });

    let subtitle = match session.has_manager {
        true => "quiz master present",
        false => "no quiz master",
    };

    html! {
        <>
            <Hero>
                <Title> {quiz.title.clone()} </Title>
                <Subtitle> {subtitle} </Subtitle>
            </Hero>

            <Hero color={Color::Primary} size={HeroSize::Medium}>
                <Container class="has-text-centered">
                    <img src={(*image).clone()} />
                    <Title> {code} </Title>
                </Container>
            </Hero>

            <Columns multiline=true centered=true class="mt-5">
                { for players }
            </Columns>
        </>
    }
}
