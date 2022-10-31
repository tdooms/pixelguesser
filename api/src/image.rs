use std::collections::HashMap;
use std::rc::Rc;

use blurhash_wasm::decode;
use reqwest::Client;
use serde::{Deserialize, Deserializer, Serialize};
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use web_sys::{window, ImageData};

use piximages::Resolution;

use crate::{download, Error, Photo, Result};
use crate::{IMAGE_PLACEHOLDER, UPLOAD_ENDPOINT};

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

// decode the blurhash and use a canvas to get the data url
fn get_url_from_blurhash(hash: &str) -> Option<String> {
    let (width, height) = (40, 30);
    let pixels = decode(hash, width, height).ok()?;

    let element = window().unwrap().document().unwrap().create_element("canvas").ok()?;
    let canvas = element.dyn_into::<web_sys::HtmlCanvasElement>().ok()?;

    canvas.set_width(width as u32);
    canvas.set_height(height as u32);

    let ctx = canvas.get_context("2d").unwrap().unwrap();
    let ctx = ctx.dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();

    let data = ImageData::new_with_u8_clamped_array(Clamped(&pixels), 0).unwrap();

    ctx.put_image_data(&data, 0.0, 0.0).ok()?;
    canvas.to_data_url().ok()
}

impl Image {
    pub async fn from_local(file: web_sys::File) -> Result<Self> {
        let blob = gloo::file::Blob::from(file.clone());
        let local = gloo::file::futures::read_as_data_url(&blob).await.unwrap();

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
            (None, None, Some(hash)) => Rc::new(get_url_from_blurhash(hash).unwrap()),
            (None, None, _) => Rc::new(IMAGE_PLACEHOLDER.to_owned()),
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
