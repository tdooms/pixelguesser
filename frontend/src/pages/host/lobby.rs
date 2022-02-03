use cobul::props::{Color, ColumnSize, HeaderSize, HeroSize};
use cobul::*;
use qrcode::QrCode;
use sessions::Session;
use std::rc::Rc;
use yew::prelude::*;

use crate::graphql::FullQuiz;
use crate::shared::SELF_ENDPOINT;
use image::Rgba;
use photon_rs::PhotonImage;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub code: String,
    pub session: Rc<Session>,
    pub quiz: Rc<FullQuiz>,
}

#[function_component(Lobby)]
pub fn lobby(props: &Props) -> Html {
    let Props { session, code, quiz } = &props;
    let url = format!("{}/manage/{}", SELF_ENDPOINT, code);

    let generate = || {
        let buffer = QrCode::new(&url)
            .unwrap()
            .render::<Rgba<u8>>()
            .light_color(Rgba([255, 255, 255, 255]))
            .build();

        let (width, height) = buffer.dimensions();
        PhotonImage::new(buffer.to_vec(), width, height).get_base64()
    };

    let image = use_state(generate);

    let players = session.players.iter().map(|(name, _)| {
        html! {
            <Column size={ColumnSize::IsNarrow}>
                <Box> <Title size={HeaderSize::Is4}> {name} </Title> </Box>
            </Column>
        }
    });

    html! {
        <>
            <Hero>
                <Title size={HeaderSize::Is1}> {quiz.title.clone()} </Title>
                <Subtitle size={HeaderSize::Is4}> {quiz.explanation.clone()} </Subtitle>
            </Hero>

            <Hero color={Color::Primary} size={HeroSize::Small}>
                <Container class="has-text-centered">
                    <img src={(*image).clone()} />
                    // <Title> </Title>
                    <p><a class="title is-3" href={url} target="_blank"> {code} </a></p>
                </Container>
            </Hero>

        <Columns centered={true}>
        <Column size={ColumnSize::Is6}>
            <Columns multiline=true centered=true class="mt-5" >
                { for players }
            </Columns>
        </Column>
        </Columns>
        </>
    }
}
