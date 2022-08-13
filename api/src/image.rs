use crate::{download, Error, Photo, Result};
use crate::{IMAGE_PLACEHOLDER, UPLOAD_ENDPOINT};
use images::Resolution;
use reqwest::Client;
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
        let url = map.remove("url").flatten();
        let blurhash = map.remove("blurhash").flatten();

        let service = match &url {
            Some(url) if url.contains("unsplash") => Some(Service::Unsplash { meta: None }),
            _ => Some(Service::Piximages),
        };

        Ok(Self { url: url.map(Rc::new), blurhash, service, ..Default::default() })
    }
}

impl Image {
    pub async fn from_local(file: web_sys::File) -> Result<Self> {
        let blob = gloo_file::Blob::from(file.clone());
        let local = gloo_file::futures::read_as_data_url(&blob).await.unwrap();

        Ok(Self { local: Some(Rc::new(local)), name: Some(file.name()), ..Default::default() })
    }

    pub fn from_unsplash(photo: &Photo) -> Self {
        let meta = Some(Unsplash {
            name: photo.user.name.clone(),
            profile: photo.user.links.html.clone(),
            download: photo.links.download_location.clone(),
        });

        let url = Some(Rc::new(photo.urls.full.clone()));
        let service = Some(Service::Unsplash { meta });
        let blurhash = Some(photo.blur_hash.clone());
        let name = photo.description.clone();

        Self { url, service, blurhash, name, ..Default::default() }
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

    pub async fn upload(&mut self, token: String) -> Result<()> {
        // https://help.unsplash.com/en/articles/2511258-guideline-triggering-a-download
        if let Some(Service::Unsplash { meta: Some(meta) }) = &self.service {
            download(meta.download.clone()).await;
            // Once the crediting is done, we can ignore the crediting to avoid duplicates
            self.service = None
        }

        if let (Some(local), None) = (&self.local, &self.url) {
            let endpoint = format!("{UPLOAD_ENDPOINT}/upload");

            let body = match local.split(',').nth(1) {
                None => (**local).clone(),
                Some(split) => split.to_owned(),
            };

            let response = Client::new()
                .post(&endpoint)
                .header("Authorization", token)
                .body(body)
                .send()
                .await?;

            (response.status() == 200).then(|| ()).ok_or(Error::ImageUpload)?;
            let url = response.text().await?;

            self.url = Some(Rc::new(url));
            self.service = Some(Service::Piximages);
        }

        Ok(())
    }
}
