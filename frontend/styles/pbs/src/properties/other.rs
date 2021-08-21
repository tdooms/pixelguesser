use derive_more::Display;
use yew::Classes;

#[derive(Clone, Debug, Display, PartialEq, Copy)]
#[display(fmt = "has-{}-separator")]
pub enum Separator {
    #[display(fmt = "arrow")]
    Arrow,
    #[display(fmt = "bullet")]
    Bullet,
    #[display(fmt = "dot")]
    Dot,
    #[display(fmt = "succeeds")]
    Succeeds,
}

impl Into<Classes> for Separator {
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
}

/// The 4 allowed types for an input component.
///
/// https://bulma.io/documentation/form/input/
#[derive(Clone, Debug, Display, PartialEq, Copy)]
pub enum InputType {
    #[display(fmt = "text")]
    Text,
    #[display(fmt = "password")]
    Password,
    #[display(fmt = "email")]
    Email,
    #[display(fmt = "tel")]
    Tel,
}

impl Default for InputType {
    fn default() -> Self {
        InputType::Text
    }
}

impl Into<Classes> for InputType {
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
}

/// Tile context modifiers.
/// https://bulma.io/documentation/layout/tiles/#modifiers
#[derive(Clone, Debug, Display, PartialEq, Copy)]
#[display(fmt = "is-{}")]
pub enum TileCtx {
    #[display(fmt = "ancestor")]
    Ancestor,
    #[display(fmt = "parent")]
    Parent,
    #[display(fmt = "child")]
    Child,
}

impl Into<Classes> for TileCtx {
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
}