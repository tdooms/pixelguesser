use derive_more::Display;
use gloo::file::futures::read_as_data_url;
use keys::IMAGE_ENDPOINT;
use reqwasm::http::Request;

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

pub enum ImageData {
    Local(String),
    Url(String),
    Both(String, String),
}

pub struct Image {
    data: ImageData,
    name: String,
}

impl Image {
    pub async fn from_local(file: &web_sys::File) -> Self {
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
            ImageData::Url(url) => format!("{}?{}", url, resolution),
        }
    }
}
