
pub fn base64(data: &[u8]) -> String {
    base64::encode(&data)
}