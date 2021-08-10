use derive_more::Display;

/// Common alignment classes.
#[derive(Clone, Debug, Display, PartialEq, Copy)]
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
/// TODO: are-* sizes for buttons https://bulma.io/documentation/elements/button/#sizes
#[derive(Clone, Debug, Display, PartialEq, Copy)]
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
#[derive(Clone, Debug, Display, PartialEq, Copy)]
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
    #[display(fmt = "warning")]
    Warning,
    #[display(fmt = "danger")]
    Danger,
}

/// https://bulma.io/documentation/helpers/color-helpers/
/// Text color classes.
/// TODO: this is the same as background colors with 'has-background-{}'
#[derive(Clone, Debug, Display, PartialEq, Copy)]
#[display(fmt = "has-text-{}")]
pub enum TextColor {
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
    #[display(fmt = "black-bis")]
    BlackBis,
    #[display(fmt = "black-ter")]
    BlackTer,
    #[display(fmt = "grey-darker")]
    GreyDarker,
    #[display(fmt = "grey-dark")]
    GreyDark,
    #[display(fmt = "grey")]
    Grey,
    #[display(fmt = "grey-light")]
    GreyLight,
    #[display(fmt = "grey-lighter")]
    GreyLighter,
    #[display(fmt = "white-ter")]
    WhiteTer,
    #[display(fmt = "white-bis")]
    WhiteBis,
    #[display(fmt = "primary-light")]
    PrimaryLight,
    #[display(fmt = "link-light")]
    LinkLight,
    #[display(fmt = "info-light")]
    InfoLight,
    #[display(fmt = "success-light")]
    SuccessLight,
    #[display(fmt = "warning-light")]
    WarningLight,
    #[display(fmt = "danger-light")]
    DangerLight,
    #[display(fmt = "primary-dark")]
    PrimaryDark,
    #[display(fmt = "link-dark")]
    LinkDark,
    #[display(fmt = "info-dark")]
    InfoDark,
    #[display(fmt = "success-dark")]
    SuccessDark,
    #[display(fmt = "warning-dark")]
    WarningDark,
    #[display(fmt = "danger-dark")]
    DangerDark,
}

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

/// Available placeholder sizes for figures.
///
/// https://bulma.io/documentation/elements/image/
#[derive(Clone, Debug, Display, PartialEq, Copy)]
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
#[derive(Clone, Debug, Display, PartialEq, Copy)]
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

/// The 2 sizes available for sections, which controls spacing.
/// [https://bulma.io/documentation/layout/section/](https://bulma.io/documentation/layout/section/)
#[derive(Clone, Debug, Display, PartialEq, Copy)]
#[display(fmt = "is-{}")]
pub enum SectionSize {
    #[display(fmt = "medium")]
    Medium,
    #[display(fmt = "large")]
    Large,
}

/// Container sizes
/// [https://bulma.io/documentation/layout/container/#widescreen-or-fullhd-only](https://bulma.io/documentation/layout/container/#widescreen-or-fullhd-only)
#[derive(Clone, Debug, Display, PartialEq, Copy)]
#[display(fmt = "is-{}")]
pub enum ContainerSize {
    #[display(fmt = "widescreen")]
    Widescreen,
    #[display(fmt = "fullhd")]
    Fullhd,
    #[display(fmt = "max-desktop")]
    MaxDesktop,
    #[display(fmt = "max-widescreen")]
    MaxWidescreen,
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

// TODO: https://bulma.io/documentation/columns/sizes/#narrow-column
// narrow breakpoint sizes
#[derive(Clone, Debug, Display, PartialEq, Copy)]
#[display(fmt = "is-{}")]
pub enum ColumnSize {
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
    #[display(fmt = "7")]
    Is7,
    #[display(fmt = "8")]
    Is8,
    #[display(fmt = "9")]
    Is9,
    #[display(fmt = "10")]
    Is10,
    #[display(fmt = "11")]
    Is11,
    #[display(fmt = "12")]
    Is12,
    #[display(fmt = "full")]
    IsFull,
    #[display(fmt = "four-fifths")]
    IsFourFifths,
    #[display(fmt = "three-quarters")]
    IsThreeQuarters,
    #[display(fmt = "two-thirds")]
    IsTwoThirds,
    #[display(fmt = "three-fifths")]
    IsThreeFifths,
    #[display(fmt = "half")]
    IsHalf,
    #[display(fmt = "two-fifths")]
    IsTwoFifths,
    #[display(fmt = "one-third")]
    IsOneThird,
    #[display(fmt = "one-quarter")]
    IsOneQuarter,
    #[display(fmt = "one-fifth")]
    IsOneFifth,
    #[display(fmt = "narrow")]
    IsNarrow,
}

#[derive(Clone, Debug, Display, PartialEq, Copy)]
#[display(fmt = "is-offset-{}")]
pub enum ColumnOffset {
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
    #[display(fmt = "7")]
    Is7,
    #[display(fmt = "8")]
    Is8,
    #[display(fmt = "9")]
    Is9,
    #[display(fmt = "10")]
    Is10,
    #[display(fmt = "11")]
    Is11,
    #[display(fmt = "12")]
    Is12,
    #[display(fmt = "full")]
    IsFull,
    #[display(fmt = "four-fifths")]
    IsFourFifths,
    #[display(fmt = "three-quarters")]
    IsThreeQuarters,
    #[display(fmt = "two-thirds")]
    IsTwoThirds,
    #[display(fmt = "three-fifths")]
    IsThreeFifths,
    #[display(fmt = "half")]
    IsHalf,
    #[display(fmt = "two-fifths")]
    IsTwoFifths,
    #[display(fmt = "one-third")]
    IsOneThird,
    #[display(fmt = "one-quarter")]
    IsOneQuarter,
    #[display(fmt = "one-fifth")]
    IsOneFifth,
}

/// Tile size modifiers.
///
/// https://bulma.io/documentation/layout/tiles/#modifiers
#[derive(Clone, Debug, Display, PartialEq, Copy)]
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

/// The 6 sizes available for heros.
///
/// [https://bulma.io/documentation/layout/hero/#sizes](https://bulma.io/documentation/layout/hero/#sizes)
/// [https://bulma.io/documentation/layout/hero/#fullheight-with-navbar](https://bulma.io/documentation/layout/hero/#fullheight-with-navbar)
#[derive(Clone, Debug, Display, PartialEq, Copy)]
#[display(fmt = "is-{}")]
pub enum HeroSize {
    #[display(fmt = "small")]
    Small,
    #[display(fmt = "medium")]
    Medium,
    #[display(fmt = "large")]
    Large,
    #[display(fmt = "halfheight")]
    HalfHeight,
    #[display(fmt = "fullheight")]
    Fullheight,
    #[display(fmt = "fullheight-with-navbar")]
    FullheightWithNavbar,
}
