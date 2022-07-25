use crate::{Error, IMAGE_ENDPOINT, IMAGE_PLACEHOLDER, UPLOAD_ENDPOINT};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::rc::Rc;

pub use images::Resolution;

#[cfg(all(feature = "wasm", not(feature = "native")))]
async fn upload(body: String, endpoint: String) -> Result<String, Error> {
    let response = gloo_net::http::Request::post(&endpoint).body(body).send().await?;
    (response.status() == 200).then(|| ()).ok_or(Error::ImageUpload)?;
    Ok(response.text().await?)
}

#[cfg(feature = "native")]
async fn upload(body: String, endpoint: String) -> Result<String, Error> {
    let response = reqwest::Client::new().post(&endpoint).body(body).send().await?;
    (response.status() == 200).then(|| ()).ok_or(Error::ImageUpload)?;
    Ok(response.text().await?)
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
        match Option::deserialize(deserializer)? {
            Some(url) => Ok(Self { format: Format::Url { url }, name: None }),
            None => Ok(Self::default()),
        }
    }
}

impl Image {
    #[cfg(feature = "wasm")]
    #[must_use]
    pub fn from_file(
        file: web_sys::File,
        callback: yew::Callback<Self>,
    ) -> gloo_file::callbacks::FileReader {
        let blob = gloo_file::Blob::from(file.clone());
        let name = Some(file.name());

        gloo_file::callbacks::read_as_data_url(&blob, move |data| {
            let format = Format::Local { data: Rc::new(data.unwrap()) };
            let image = Self { format, name };
            callback.emit(image)
        })
    }

    pub fn from_base64(base64: String, name: Option<String>) -> Self {
        Self { format: Format::Local { data: Rc::new(base64) }, name }
    }

    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    #[cfg(any(feature = "wasm", feature = "native"))]
    pub async fn upload(&mut self, creator: String) -> Result<(), Error> {
        if let Format::Local { data } = self.format.clone() {
            let endpoint = format!("{UPLOAD_ENDPOINT}/upload/{creator}");
            let string = match data.split(',').nth(1) {
                None => (*data).clone(),
                Some(split) => split.to_owned(),
            };
            let url = upload(string, endpoint).await?;

            #[cfg(feature = "wasm")]
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
