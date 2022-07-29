use crate::UPLOAD_ENDPOINT;
use crate::{Error, Result};
use images::Resolution;
use serde::{Deserialize, Serialize};
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

#[cfg(feature = "wasm")]
async fn upload(body: String, endpoint: String) -> Result<String> {
    let response = gloo_net::http::Request::post(&endpoint).body(body).send().await?;
    (response.status() == 200).then(|| ()).ok_or(Error::ImageUpload)?;
    Ok(response.text().await?)
}

#[cfg(feature = "native")]
async fn upload(body: String, endpoint: String) -> Result<String> {
    let response = reqwest::Client::new().post(&endpoint).body(body).send().await?;
    (response.status() == 200).then(|| ()).ok_or(Error::ImageUpload)?;
    Ok(response.text().await?)
}

#[cfg(feature = "wasm")]
async fn download(endpoint: String) {
    gloo_net::http::Request::get(&endpoint).send().await.unwrap();
}

#[cfg(feature = "native")]
async fn download(endpoint: String) {
    reqwest::get(&endpoint).await.unwrap();
}

fn piximages_service() -> Option<Service> {
    Some(Service::Piximages)
}

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Image {
    #[serde(rename = "image")]
    url: Option<Rc<String>>,
    blurhash: Option<String>,

    #[serde(skip)]
    #[serde(default = "piximages_service")]
    service: Option<Service>,

    #[serde(skip)]
    local: Option<Rc<String>>,

    #[serde(skip)]
    name: Option<String>,
}

impl Image {
    #[cfg(feature = "native")]
    pub fn from_native(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let name = path.as_ref().to_str().unwrap().to_owned();
        let bytes = std::fs::read(path).map_err(|_| Error::ImageRead(name.clone()))?;
        let base64 = base64::encode(&bytes);

        Ok(Self { local: Some(Rc::new(base64)), name: Some(name), ..Default::default() })
    }

    #[cfg(feature = "wasm")]
    pub async fn from_web(file: web_sys::File) -> Result<Self> {
        let blob = gloo_file::Blob::from(file.clone());
        let local = gloo_file::futures::read_as_data_url(&blob).await.unwrap();

        Ok(Self { local: Some(Rc::new(local)), name: Some(file.name()), ..Default::default() })
    }

    #[cfg(feature = "wasm")]
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
            (None, None, _) => Rc::new(String::new()),
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
