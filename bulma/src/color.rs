#[derive(Clone, Debug)]
pub enum Color {
    None,
    White,
    Light,
    Dark,
    Black,
    Primary,
    Link,
    Info,
    Success,
    Danger,
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Color::None => "",
            Color::Primary => "is-primary",
            Color::Link => "is-link",
            Color::Info => "is-info",
            Color::Success => "is-success",
            Color::Danger => "is-danger",
            Color::White => "is-white",
            Color::Light => "is-light",
            Color::Dark => "is-light",
            Color::Black => "is-black",
        }
        .to_owned()
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::None
    }
}
