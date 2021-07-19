use derive_more::Display;

/// Common alignment classes.
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "is-{}")]
pub enum Alignment {
    #[display(fmt = "left")]
    Left,
    #[display(fmt = "centered")]
    Centered,
    #[display(fmt = "right")]
    Right,
}

/// Common size classes.
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "is-{}")]
pub enum Size {
    #[display(fmt = "small")]
    Small,
    #[display(fmt = "normal")]
    Normal,
    #[display(fmt = "medium")]
    Medium,
    #[display(fmt = "large")]
    Large,
}

/// Common color classes.
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "is-{}")]
pub enum Color {
    #[display(fmt = "white")]
    White,
    #[display(fmt = "light")]
    Light,
    #[display(fmt = "dark")]
    Dark,
    #[display(fmt = "black")]
    Black,
    #[display(fmt = "primary")]
    Primary,
    #[display(fmt = "link")]
    Link,
    #[display(fmt = "info")]
    Info,
    #[display(fmt = "success")]
    Success,
    #[display(fmt = "danger")]
    Danger,
}

#[derive(Clone, Debug, Display, PartialEq)]
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
