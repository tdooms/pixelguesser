use derive_more::Display;
use yew::Classes;

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

impl Into<Classes> for Size {
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
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

impl Into<Classes> for ImageSize {
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
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

impl Into<Classes> for HeaderSize {
    fn into(self) -> Classes {
        Classes::from(self.to_string())
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

impl Into<Classes> for SectionSize {
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
}

/// Container sizes
/// [https://bulma.io/documentation/layout/container/#widescreen-or-fullhd-only](https://bulma.io/documentation/layout/container/#widescreen-or-fullhd-only)
#[derive(Clone, Debug, Display, PartialEq, Copy)]
#[display(fmt = "is-{}")]
pub enum ContainerSize {
    #[display(fmt = "widescreen")]
    Widescreen,
    #[display(fmt = "fullhd")]
    FullHD,
    #[display(fmt = "max-desktop")]
    MaxDesktop,
    #[display(fmt = "max-widescreen")]
    MaxWidescreen,
}

impl Into<Classes> for ContainerSize {
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
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
    #[display(fmt = "narrow-mobile")]
    IsNarrowMobile,
    #[display(fmt = "narrow-tablet")]
    IsNarrowTablet,
    #[display(fmt = "narrow-touch")]
    IsNarrowTouch,
    #[display(fmt = "narrow-desktop")]
    IsNarrowDesktop,
    #[display(fmt = "narrow-widescreen")]
    IsNarrowWidescreen,
    #[display(fmt = "narrow-fullhd")]
    IsNarrowFullHD,
}

impl Into<Classes> for ColumnSize {
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
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

impl Into<Classes> for TileSize {
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
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

impl Into<Classes> for HeroSize {
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
}