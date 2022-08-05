use crate::{Error, Result};
use crate::{IMAGE_PLACEHOLDER, UPLOAD_ENDPOINT};
use images::Resolution;
use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Unsplash {
    name: String,
    profile: String,
    download: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Service {
    Unsplash { meta: Option<Unsplash> },
    Piximages,
}

async fn upload(body: String, endpoint: String) -> Result<String> {
    let response = reqwest::Client::new().post(&endpoint).body(body).send().await?;
    (response.status() == 200).then(|| ()).ok_or(Error::ImageUpload)?;
    Ok(response.text().await?)
}

async fn download(endpoint: String) {
    reqwest::get(&endpoint).await.unwrap();
}

#[derive(Default, Serialize, Clone, Debug, PartialEq)]
pub struct Image {
    url: Option<Rc<String>>,
    blurhash: Option<String>,

    #[serde(skip)]
    service: Option<Service>,

    #[serde(skip)]
    local: Option<Rc<String>>,

    #[serde(skip)]
    name: Option<String>,
}

impl<'de> Deserialize<'de> for Image {
    fn deserialize<D: Deserializer<'de>>(deser: D) -> std::result::Result<Self, D::Error> {
        let mut map: HashMap<String, Option<String>> = HashMap::deserialize(deser)?;
        let url = map.remove("url").ok_or(D::Error::custom("image must have a url"))?.unwrap();
        let blurhash = map.remove("blurhash").flatten();

        let service = match url.contains("unsplash") {
            true => Some(Service::Unsplash { meta: None }),
            false => Some(Service::Piximages),
        };
        Ok(Self { url: Some(Rc::new(url)), blurhash, service, ..Default::default() })
    }
}

impl Image {
    pub async fn from_local(file: web_sys::File) -> Result<Self> {
        let blob = gloo_file::Blob::from(file.clone());
        let local = gloo_file::futures::read_as_data_url(&blob).await.unwrap();

        Ok(Self { local: Some(Rc::new(local)), name: Some(file.name()), ..Default::default() })
    }

    pub fn from_base64(base64: String, name: Option<String>) -> Self {
        Self { local: Some(Rc::new(base64)), name, ..Default::default() }
    }

    pub fn src(&self, resolution: Resolution) -> Rc<String> {
        let suffix = match self.service {
            Some(Service::Piximages) => format!("/{}", resolution),
            Some(Service::Unsplash { .. }) => format!("&h={}", resolution as u64),
            None => String::new(),
        };

        match (&self.local, &self.url, &self.blurhash) {
            (Some(local), _, _) => Rc::clone(local),
            (None, Some(url), _) => Rc::new(format!("{}{suffix}", url.as_ref())),
            (None, None, _) => Rc::new(IMAGE_PLACEHOLDER.to_owned()),
            // TODO: https://github.com/vaalentin/array-to-image/blob/master/src/index.js
            // Uint8ClampedArray
        }
    }

    pub fn url(&self) -> Option<Rc<String>> {
        self.url.clone()
    }

    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn is_empty(&self) -> bool {
        self.local.is_none() && self.url.is_none()
    }

    pub async fn upload(&mut self, creator: String) -> Result<()> {
        // https://help.unsplash.com/en/articles/2511258-guideline-triggering-a-download
        if let Some(Service::Unsplash { meta: Some(meta) }) = &self.service {
            download(meta.download.clone()).await;
            // Once the crediting is done, we can ignore the crediting toa void duplicates
            self.service = None
        }

        if let (Some(local), None) = (&self.local, &self.url) {
            let endpoint = format!("{UPLOAD_ENDPOINT}/upload/{creator}");
            let string = match local.split(',').nth(1) {
                None => (**local).clone(),
                Some(split) => split.to_owned(),
            };
            let url = upload(string, endpoint).await?;
            self.url = Some(Rc::new(url));
            self.service = Some(Service::Piximages);
        }

        Ok(())
    }
}
