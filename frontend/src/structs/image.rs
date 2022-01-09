use crate::constants::IMAGE_ENDPOINT;
use crate::error::Error;
use reqwasm::http::Request;
use serde::{Serialize, Serializer};
use std::cell::RefCell;
use std::fmt::Debug;
use web_sys::Url;

#[derive(Clone, Debug, PartialEq)]
pub enum ImageData {
    Local { data: String, name: String, url: RefCell<Option<String>> },
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

impl ImageData {
    pub fn from_local(file: &web_sys::File) -> Option<Self> {
        Some(Self::Local {
            data: Url::create_object_url_with_blob(file).ok()?,
            name: file.name(),
            url: RefCell::default(),
        })
    }

    pub fn from_url(url: String) -> Self {
        Self::Url(url)
    }

    pub fn src(&self) -> String {
        match self {
            ImageData::Local { data, .. } => data.clone(),
            ImageData::Url(url) => url.clone(),
        }
    }

    pub async fn upload(&self) -> Result<(), Error> {
        match self {
            ImageData::Local { data, url, .. } => match url.clone().take() {
                None => Err(Error::Reupload),
                Some(_) => {
                    let stripped = data.split(',').nth(1).unwrap();

                    // TODO: how is From<Vec<u8>> not implemented for JsValue?
                    let response =
                        Request::post(IMAGE_ENDPOINT).body(stripped).send().await.unwrap();
                    let filename = response.text().await.unwrap();

                    url.replace(Some(filename));
                    Ok(())
                }
            },
            ImageData::Url(_) => Err(Error::Reupload),
        }
    }

    pub fn name(&self) -> String {
        match &self {
            ImageData::Local { name, .. } => name.clone(),
            ImageData::Url(url) => url.clone(),
        }
    }
}
