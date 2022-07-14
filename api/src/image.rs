use crate::{Error, IMAGE_ENDPOINT, IMAGE_PLACEHOLDER, UPLOAD_ENDPOINT};
use gloo::file::futures::read_as_data_url;
use reqwasm::http::Request;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::rc::Rc;
use yew::Callback;

#[derive(Clone, Copy, Debug, PartialEq, derive_more::Display)]
pub enum Resolution {
    #[display(fmt = "thumbnail")]
    Thumbnail = 108,
    #[display(fmt = "card")]
    Card = 320,
    #[display(fmt = "hd")]
    HD = 1080,
    #[display(fmt = "original")]
    Original,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Format {
    Both {
        data: Rc<String>,
        url: String,
    },
    Local {
        data: Rc<String>,
    },
    Url {
        url: String,
    },

    #[default]
    None,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Image {
    format: Format,
    name: Option<String>,
}

impl Serialize for Image {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match &self.format {
            Format::None => serializer.serialize_none(),
            Format::Local { .. } => Err(serde::ser::Error::custom("Must have a url")),
            Format::Url { url } | Format::Both { url, .. } => serializer.serialize_str(url),
        }
    }
}

impl<'de> Deserialize<'de> for Image {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let url = String::deserialize(deserializer)?;
        Ok(Self { format: Format::Url { url }, name: None })
    }
}

impl Image {
    pub fn from_file(file: web_sys::File, callback: Callback<Self>) {
        let blob = gloo::file::Blob::from(file.clone());
        wasm_bindgen_futures::spawn_local(async move {
            let data = Rc::new(read_as_data_url(&blob).await.unwrap());
            let result = Self { format: Format::Local { data }, name: Some(file.name()) };
            callback.emit(result)
        })
    }

    pub fn from_base64(base64: String, name: Option<String>) -> Self {
        Self { format: Format::Local { data: Rc::new(base64) }, name }
    }

    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    pub async fn upload(&mut self) -> Result<(), Error> {
        if let Format::Local { data } = self.format.clone() {
            let endpoint = format!("{UPLOAD_ENDPOINT}/upload");

            let body = data.split(',').nth(1).unwrap().to_owned();
            let response = Request::post(&endpoint).body(body).send().await.unwrap();
            (response.status() == 200).then(|| ()).ok_or(Error::Upload)?;

            let url = response.text().await?;
            log::trace!("uploaded image filename: {}", url);
            self.format = Format::Both { data, url };
        }
        Ok(())
    }

    pub fn src(&self, resolution: Resolution) -> Rc<String> {
        match &self.format {
            Format::None => Rc::new(IMAGE_PLACEHOLDER.to_string()),
            Format::Local { data } | Format::Both { data, .. } => data.clone(),
            Format::Url { url } => Rc::new(format!("{IMAGE_ENDPOINT}/{resolution}/{url}")),
        }
    }

    pub fn url(&self) -> Option<String> {
        match &self.format {
            Format::None | Format::Local { .. } => None,
            Format::Url { url } | Format::Both { url, .. } => Some(url.clone()),
        }
    }

    pub fn is_none(&self) -> bool {
        match self.format {
            Format::None => true,
            _ => false,
        }
    }
}
