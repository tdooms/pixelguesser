// use crate::error::Error;
// use gloo::file::futures::read_as_data_url;
// use keys::IMAGE_ENDPOINT;
// use reqwasm::http::Request;
// use serde::{Deserialize, Deserializer, Serialize, Serializer};
// use std::fmt::{Debug, Display, Formatter};
// use web_sys::{File, Url};
//
// #[derive(Clone, Debug)]
// pub enum Image {
//     Local { file: File, src: String, url: Option<String> },
//     Url(String),
// }
//
// impl PartialEq for Image {
//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//             (Image::Local { src: x, .. }, Image::Local { src: y, .. }) => x.eq(y),
//             (Image::Url(x), Image::Url(y)) => x.eq(y),
//             (Image::Local { .. }, Image::Url(_)) => true,
//             (Image::Url(_), Image::Local { .. }) => false,
//         }
//     }
// }
//
// impl<'de> Deserialize<'de> for Image {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let url = String::deserialize(deserializer)?;
//         Ok(Self::Url(url))
//     }
// }
//
// impl Serialize for Image {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         match self {
//             Image::Local { url, .. } => match url {
//                 Some(url) => serializer.serialize_str(&url),
//                 None => unimplemented!("howto return value"),
//             },
//             Image::Url(url) => serializer.serialize_str(&url),
//         }
//     }
// }
//
// impl Display for Image {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Image::Local { url, .. } => match url {
//                 Some(url) => f.write_str(&url),
//                 None => Ok(()),
//             },
//             Image::Url(url) => f.write_str(url),
//         }
//     }
// }
//
// impl Image {
//     pub fn from_local(file: &File) -> Option<Self> {
//         let src = Url::create_object_url_with_blob(&file).ok().unwrap_or_default();
//         Some(Self::Local { src, file: file.clone(), url: None })
//     }
//
//     pub fn from_url(url: String) -> Self {
//         Self::Url(url.clone())
//     }
//
//     pub fn src(&self) -> String {
//         match self {
//             Image::Local { src, .. } => src.clone(),
//             Image::Url(url) => format!("{}/{}", IMAGE_ENDPOINT, url),
//         }
//     }
//
//     pub fn url(&self) -> Option<String> {
//         match self {
//             Image::Local { url: Some(url), .. } => Some(url.clone()),
//             Image::Url(url) => Some(url.clone()),
//             _ => None,
//         }
//     }
//
//     pub async fn upload(&mut self) -> Result<(), Error> {
//         match self {
//             Image::Local { file, url, .. } => match url.clone().take() {
//                 None => {
//                     let blob = gloo::file::Blob::from(file.clone());
//                     let data = read_as_data_url(&blob).await.unwrap();
//                     let stripped = data.split(',').nth(1).unwrap();
//
//                     // TODO: how is From<Vec<u8>> not implemented for JsValue?
//                     let endpoint = format!("{}/upload", IMAGE_ENDPOINT);
//                     let response = Request::post(&endpoint).body(stripped).send().await?;
//
//                     let filename = response.text().await.unwrap();
//                     *url = Some(filename.clone());
//
//                     // Return the new url to be submitted for change
//                     Ok(())
//                 }
//                 // this means already sent
//                 Some(_) => Ok(()),
//             },
//             // This means there is no data to send as it hasn't changed
//             Image::Url(_) => Ok(()),
//         }
//     }
//
//     pub fn name(&self) -> String {
//         match &self {
//             Image::Local { file, .. } => file.name(),
//             Image::Url(url) => url.clone(),
//         }
//     }
// }
