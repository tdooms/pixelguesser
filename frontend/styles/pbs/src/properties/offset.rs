use derive_more::Display;
use yew::Classes;

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

impl Into<Classes> for ColumnOffset {
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
}