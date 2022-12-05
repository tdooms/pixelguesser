#[cfg(feature = "auth")]
pub use crate::pixauth::*;
#[cfg(feature = "auth")]
pub use ::pixauth::*;

#[cfg(feature = "sessions")]
pub use crate::pixessions::*;
#[cfg(feature = "sessions")]
pub use ::pixessions::*;

#[cfg(feature = "images")]
pub use crate::piximages::*;
#[cfg(feature = "images")]
pub use ::piximages::Resolution;

pub use consts::*;
pub use error::{Error, Result};

#[cfg(feature = "graphql")]
pub use quiz::*;
#[cfg(feature = "graphql")]
pub use round::*;
#[cfg(feature = "graphql")]
pub use user::*;

#[cfg(feature = "images")]
pub use image::Image;
#[cfg(feature = "images")]
pub use unsplash::*;

mod consts;
mod error;

#[cfg(feature = "auth")]
mod pixauth;
#[cfg(feature = "sessions")]
mod pixessions;

#[cfg(feature = "images")]
mod image;
#[cfg(feature = "images")]
mod piximages;
#[cfg(feature = "images")]
mod unsplash;

#[cfg(feature = "graphql")]
mod quiz;
#[cfg(feature = "graphql")]
mod round;
#[cfg(feature = "graphql")]
mod user;
