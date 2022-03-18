use crate::IMAGE_ENDPOINT;
use derive_more::Display;
use gloo::file::futures::read_as_data_url;
use reqwasm::http::Request;
use serde::de::Error;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Formatter;

#[derive(Display)]
pub enum Resolution {
    #[display(fmt = "height=108")]
    Thumbnail,
    #[display(fmt = "height=324")]
    Card,
    #[display(fmt = "height=1080")]
    FullHd,
    #[display(fmt = "")]
    Max,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ImageData {
    Local(String),
    Url(String),
    Both(String, String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Image {
    data: ImageData,
    name: String,
}

impl Serialize for Image {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.data {
            ImageData::Local(_) => unimplemented!(),
            ImageData::Url(url) | ImageData::Both(url, _) => serializer.serialize_str(url),
        }
    }
}

impl<'de> Deserialize<'de> for Image {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let url = deserializer.deserialize_str(StrVisitor)?;
        Ok(Self { data: ImageData::Url(url), name: "".to_string() })
    }
}

struct StrVisitor;
impl<'de> Visitor<'de> for StrVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v.to_owned())
    }
}

impl Image {
    pub async fn from_local(file: web_sys::File) -> Self {
        let blob = gloo::file::Blob::from(file.clone());
        let data = read_as_data_url(&blob).await.unwrap();

        Image { data: ImageData::Local(data), name: file.name() }
    }

    pub fn from_url(url: impl ToString) -> Self {
        Image { data: ImageData::Url(url.to_string()), name: String::default() }
    }

    pub async fn upload(&mut self) {
        if let ImageData::Local(local) = &self.data {
            let endpoint = format!("{}/upload", IMAGE_ENDPOINT);
            log::warn!("{}", endpoint);
            let response = Request::post(&endpoint).body(local.clone()).send().await.unwrap();

            let filename = response.text().await.unwrap();
            self.data = ImageData::Both(local.clone(), filename)
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn src(&self, resolution: Resolution) -> String {
        match &self.data {
            ImageData::Local(src) | ImageData::Both(src, _) => src.clone(),
            ImageData::Url(url) => format!("{}/{}?{}", IMAGE_ENDPOINT, url, resolution),
        }
    }
}
