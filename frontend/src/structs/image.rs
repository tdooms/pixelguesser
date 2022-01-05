use serde::de::{DeserializeOwned, Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Formatter, Write};
use web_sys::Url;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Image {
    local: Option<web_sys::File>,

    url: Option<String>,
}

struct ImageVisitor;

impl<'de> Visitor for ImageVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a valid string that represents the image url")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v.to_owned())
    }
}

impl<'de> Deserialize<'de> for Image {
    fn deserialize<D>(deserializer: D) -> Result<Self, serde::de::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ImageVisitor)
    }
}

impl Serialize for Image {
    fn serialize<S>(&self, serializer: S) -> Result<serde::ser::Ok, serde::ser::Error>
    where
        S: Serializer,
    {
        match &self.url {
            Some(url) => serializer.serialize_str(url.as_str()),
            None => Ok(serde::ser::Ok),
        }
    }
}

impl Image {
    pub fn from_local(file: &web_sys::File) -> Self {
        Self { local: Some(file.clone()), url: Url::create_object_url_with_blob(file).ok() }
    }

    pub fn from_url(url: &str) -> Self {
        Self { local: None, url: Some(url.to_owned()) }
    }

    pub fn src(&self) -> Option<String> {
        self.url.clone()
    }

    pub fn name(&self) -> Option<String> {
        match (&self.url, &self.local) {
            (Some(url), _) => Some(url.clone()),
            (_, Some(local)) => Some(local.name()),
            (None, None) => None,
        }
    }
}
