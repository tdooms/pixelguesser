use derive_more::Display;
use yew::Classes;

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

impl Into<Classes> for Color {
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
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

impl Into<Classes> for TextColor {
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
}