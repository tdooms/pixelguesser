pub fn bytes_to_url<T: AsRef<[u8]>>(bytes: T) -> String {
    format!("data:image/png;base64,{}", base64::encode(bytes))
}
