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

impl Default for Alignment {
    fn default() -> Self {
        Alignment::Left
    }
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

impl Default for Size {
    fn default() -> Self {
        Size::Normal
    }
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

/// Available placeholder sizes for figures.
///
/// https://bulma.io/documentation/elements/image/
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "is-{}")]
pub enum ImageSize {
    #[display(fmt = "16x16")]
    Is16x16,
    #[display(fmt = "24x24")]
    Is24x24,
    #[display(fmt = "32x32")]
    Is32x32,
    #[display(fmt = "48x48")]
    Is48x48,
    #[display(fmt = "64x64")]
    Is64x64,
    #[display(fmt = "96x96")]
    Is96x96,
    #[display(fmt = "128x128")]
    Is128x128,
    #[display(fmt = "Square")]
    IsSquare,
    #[display(fmt = "1by1")]
    Is1by1,
    #[display(fmt = "5by4")]
    Is5by4,
    #[display(fmt = "4by3")]
    Is4by3,
    #[display(fmt = "3by2")]
    Is3by2,
    #[display(fmt = "5by3")]
    Is5by3,
    #[display(fmt = "16by9")]
    Is16by9,
    #[display(fmt = "2by1")]
    Is2by1,
    #[display(fmt = "3by1")]
    Is3by1,
    #[display(fmt = "4by5")]
    Is4by5,
    #[display(fmt = "3by4")]
    Is3by4,
    #[display(fmt = "2by3")]
    Is2by3,
    #[display(fmt = "3by5")]
    Is3by5,
    #[display(fmt = "9by16")]
    Is9by16,
    #[display(fmt = "1by2")]
    Is1by2,
    #[display(fmt = "1by3")]
    Is1by3,
}

/// The six sizes available for titles & subtitles.
///
/// https://bulma.io/documentation/elements/title/#sizes
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "is-{}")]
pub enum HeaderSize {
    #[display(fmt = "1")]
    Is1,
    #[display(fmt = "2")]
    Is2,
    #[display(fmt = "3")]
    Is3,
    #[display(fmt = "4")]
    Is4,
    #[display(fmt = "5")]
    Is5,
    #[display(fmt = "6")]
    Is6,
}

/// The 4 allowed types for an input component.
///
/// https://bulma.io/documentation/form/input/
#[derive(Clone, Debug, Display, PartialEq)]
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

/// The 2 sizes available for sections, which controls spacing.
///
/// [https://bulma.io/documentation/layout/section/](https://bulma.io/documentation/layout/section/)
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "is-{}")]
pub enum SectionSize {
    #[display(fmt = "medium")]
    Medium,
    #[display(fmt = "large")]
    Large,
}

/// Tile context modifiers.
///
/// https://bulma.io/documentation/layout/tiles/#modifiers
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "is-{}")]
pub enum TileCtx {
    #[display(fmt = "ancestor")]
    Ancestor,
    #[display(fmt = "parent")]
    Parent,
    #[display(fmt = "child")]
    Child,
}

/// Tile size modifiers.
///
/// https://bulma.io/documentation/layout/tiles/#modifiers
#[derive(Clone, Debug, Display, PartialEq)]
#[display(fmt = "is-{}")]
pub enum TileSize {
    #[display(fmt = "1")]
    One,
    #[display(fmt = "2")]
    Two,
    #[display(fmt = "3")]
    Three,
    #[display(fmt = "4")]
    Four,
    #[display(fmt = "5")]
    Five,
    #[display(fmt = "6")]
    Six,
    #[display(fmt = "7")]
    Seven,
    #[display(fmt = "8")]
    Eight,
    #[display(fmt = "9")]
    Nine,
    #[display(fmt = "10")]
    Ten,
    #[display(fmt = "11")]
    Eleven,
    #[display(fmt = "12")]
    Twelve,
}
