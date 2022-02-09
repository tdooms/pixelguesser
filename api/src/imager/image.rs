use gloo::file::futures::read_as_data_url;
use gloo::file::Blob;
use reqwasm::http::Request;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Display, Formatter};
use web_sys::Url;

use crate::shared::{Error, IMAGE_ENDPOINT};

#[derive(Clone, Debug)]
pub enum Image {
    Local { file: web_sys::File, src: String, url: Option<String> },
    Url(String),
}

impl PartialEq for ImageData {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ImageData::Local { src: x, .. }, ImageData::Local { src: y, .. }) => x.eq(y),
            (ImageData::Url(x), ImageData::Url(y)) => x.eq(y),
            (ImageData::Local { .. }, ImageData::Url(_)) => true,
            (ImageData::Url(_), ImageData::Local { .. }) => false,
        }
    }
}

impl<'de> Deserialize<'de> for ImageData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let url = String::deserialize(deserializer)?;
        Ok(Self::Url(url))
    }
}

impl Serialize for ImageData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ImageData::Local { url, .. } => match url {
                Some(url) => serializer.serialize_str(&url),
                None => unimplemented!("howto return value"),
            },
            ImageData::Url(url) => serializer.serialize_str(&url),
        }
    }
}

impl Display for ImageData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageData::Local { url, .. } => match url {
                Some(url) => f.write_str(&url),
                None => Ok(()),
            },
            ImageData::Url(url) => f.write_str(url),
        }
    }
}

impl ImageData {
    pub fn from_local(file: &web_sys::File) -> Option<Self> {
        let src = Url::create_object_url_with_blob(&file).ok().unwrap_or_default();
        Some(Self::Local { src, file: file.clone(), url: None })
    }

    pub fn from_url(url: String) -> Self {
        Self::Url(url.clone())
    }

    pub fn src(&self) -> String {
        match self {
            ImageData::Local { src, .. } => src.clone(),
            ImageData::Url(url) => format!("{}/{}", IMAGE_ENDPOINT, url),
        }
    }

    pub async fn upload(&mut self) -> Result<String, ()> {
        match self {
            ImageData::Local { file, url, .. } => match url.clone().take() {
                None => {
                    let blob = Blob::from(file.clone());
                    let data = read_as_data_url(&blob).await.unwrap();
                    let stripped = data.split(',').nth(1).unwrap();

                    // TODO: how is From<Vec<u8>> not implemented for JsValue?
                    let endpoint = format!("{}/upload", IMAGE_ENDPOINT);
                    let response = Request::post(&endpoint).body(stripped).send().await.unwrap();

                    let filename = response.text().await.unwrap();
                    *url = Some(filename);

                    // Return the new url to be submitted for change
                    Ok(filename)
                }
                // this means already sent
                Some(_) => Err(()),
            },
            // This means there is no data to send as it hasn't changed
            ImageData::Url(_) => Err(()),
        }
    }

    pub fn name(&self) -> String {
        match &self {
            ImageData::Local { file, .. } => file.name(),
            ImageData::Url(url) => url.clone(),
        }
    }
}
