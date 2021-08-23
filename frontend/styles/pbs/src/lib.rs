#![cfg_attr(feature = "docinclude", feature(external_doc))]
#![cfg_attr(feature = "docinclude", doc(include = "../README.md"))]
#![recursion_limit = "1024"]

pub mod components;
pub mod elements;
pub mod form;
pub mod layout;
pub mod properties;

pub mod prelude {
    pub use crate::components::*;
    pub use crate::elements::*;
    pub use crate::form::*;
    pub use crate::layout::*;
}
