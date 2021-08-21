use std::marker::PhantomData;
use std::ops::Deref;

use wasm_bindgen::JsValue;
use yew::NodeRef;
use yew::web_sys::Node;

use crate::structs::Error;

pub struct TypeRef<T: AsRef<Node> + From<JsValue>> {
    node: NodeRef,
    phantom: PhantomData<T>,
}

impl<T: AsRef<Node> + From<JsValue>> TypeRef<T> {
    pub fn get(&self) -> Result<T, Error> {
        self.node.cast::<T>().ok_or(Error::InvalidCast)
    }
}

impl<T: AsRef<Node> + From<JsValue>> Default for TypeRef<T> {
    fn default() -> Self {
        Self { node: NodeRef::default(), phantom: Default::default() }
    }
}

impl<T: AsRef<Node> + From<JsValue>> Deref for TypeRef<T> {
    type Target = NodeRef;

    fn deref(&self) -> &Self::Target {
        &self.node
    }
}