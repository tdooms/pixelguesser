#[derive(Clone, Copy, Debug, PartialEq, derive_more::Display, strum::EnumIter)]
pub enum Resolution {
    #[display(fmt = "original")]
    Original = 0,
    #[display(fmt = "thumb")]
    Thumb = 108,
    #[display(fmt = "small")]
    Small = 320,
    #[display(fmt = "hd")]
    HD = 1080,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq)]
pub struct Response {
    pub url: String,
    pub blurhash: String,
}
