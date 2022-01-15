use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter};

use gloo::file::futures::read_as_data_url;
use gloo::file::Blob;
use reqwasm::http::Request;
use serde::{Serialize, Serializer};
use web_sys::Url;

use crate::constants::IMAGE_ENDPOINT;
use crate::error::Error;

#[derive(Clone, Debug, PartialEq)]
pub enum ImageData {
    Local { file: web_sys::File, src: String, url: RefCell<Option<String>> },
    Url(String),
}

impl Serialize for ImageData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ImageData::Local { url, .. } => match url.borrow().as_ref() {
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
            ImageData::Local { url, .. } => match url.borrow().as_ref() {
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
        Some(Self::Local { src, file: file.clone(), url: RefCell::default() })
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

    pub async fn upload(&self) -> Result<(), Error> {
        match self {
            ImageData::Local { file, url, .. } => match url.clone().take() {
                None => {
                    let data = read_as_data_url(&Blob::from(file.clone())).await.unwrap();
                    let stripped = data.split(',').nth(1).unwrap();

                    // TODO: how is From<Vec<u8>> not implemented for JsValue?
                    let response = Request::post(&format!("{}/upload", IMAGE_ENDPOINT))
                        .body(stripped)
                        .send()
                        .await
                        .unwrap();
                    let filename = response.text().await.unwrap();

                    url.replace(Some(filename));
                    Ok(())
                }
                Some(_) => Err(Error::Reupload),
            },
            ImageData::Url(_) => Err(Error::Reupload),
        }
    }

    pub fn name(&self) -> String {
        match &self {
            ImageData::Local { file, .. } => file.name(),
            ImageData::Url(url) => url.clone(),
        }
    }
}
