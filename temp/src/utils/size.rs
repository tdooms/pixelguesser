#[derive(Debug, Clone, Copy)]
pub enum Size {
    Small,
    Normal,
    Medium,
    Large,
}

impl ToString for Size {
    fn to_string(&self) -> String {
        match self {
            Size::Small => "is-small",
            Size::Normal => "is-normal",
            Size::Medium => "is-medium",
            Size::Large => "is-large",
        }
        .to_owned()
    }
}

impl Default for Size {
    fn default() -> Self {
        Self::Normal
    }
}
