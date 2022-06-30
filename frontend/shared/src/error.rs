// #[derive(thiserror::Error, Debug)]
// pub enum Error {
//     #[error("Api error {0}")]
//     Api(#[from] api::Error),
//
//     #[error("Connection error: {0}")]
//     WebSocket(String),
//
//     #[error("Could not cast to the specified html element")]
//     InvalidCast,
//
//     #[error("Pixelation component could not load image")]
//     ImageCouldNotLoad,
//
//     #[error("Error executing javascript code")]
//     JsError,
//
//     #[error("Encountered an error while drawing on a canvas")]
//     DrawError,
//
//     #[error("Cannot select multiple files as image")]
//     MultipleFiles,
//
//     #[error("Error while initializing authentication: {0}")]
//     AuthInit(String),
//
//     #[error("Could not deserialize user: {0}")]
//     UserSerde(#[from] serde_wasm_bindgen::Error),
//
//     #[error("Quiz must have at least one round")]
//     DeleteOnlyRound,
//
//     #[error("Rounds are not linked to any quiz")]
//     UnlinkedRounds,
//
//     #[error("Cannot delete a quiz that is still being created")]
//     DeleteUncommittedQuiz,
//
//     #[error("No quiz with given id")]
//     QuizNotFound,
//
//     #[error("Must be authenticated to perform this action")]
//     NotAuthenticated,
// }

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Api(#[from] api::Error),

    #[error("{0}")]
    Internal(#[from] Internal),

    #[error("{0}")]
    Warning(#[from] Warning),

    #[error("{0}")]
    Info(#[from] Info),
}

// Unsolvable errors
#[derive(thiserror::Error, Debug)]
pub enum Internal {
    #[error("the sessions server is unreachable")]
    SessionsUnreachable,

    #[error("the images server is unreachable")]
    ImagesUnreachable,

    #[error("hasura is unreachable")]
    HasuraUnreachable,

    #[error("auth0 is unreachable")]
    AuthUnreachable,

    #[error("auth serde error")]
    AuthError(#[from] serde_wasm_bindgen::Error),

    #[error("invalid javascript cast")]
    InvalidCast,

    #[error("generic javascript error")]
    JsError,

    #[error("javascript drawing error")]
    DrawError,

    #[error("image could not load")]
    ImageCouldNotLoad,
}

// Could be bad
#[derive(thiserror::Error, Debug)]
pub enum Warning {
    #[error("multiple files selected")]
    MultipleFiles,
}

/// Requires user intervention
#[derive(thiserror::Error, Debug)]
pub enum Info {
    #[error("host left the session")]
    HostLeft,

    #[error("manager has left the session")]
    ManagerLeft,
}

// use cobul::*;
// use yew::*;
//
// pub trait Alert {
//     fn alert(&self) -> Html;
// }
//
// struct HostLeft;
// impl Alert for HostLeft {
//     fn alert(&self) -> Html {
//         let footer = html! { <Button> <p> {"Leave"} </p> </Button> };
//         html! {
//             <ModalCard title="Host has left the session" {footer}>
//                 <p> {"This means you cannot continue playing this session"} </p>
//             </ModalCard>
//         }
//     }
// }
