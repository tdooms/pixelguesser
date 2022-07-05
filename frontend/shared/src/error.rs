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
