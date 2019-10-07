pub fn exif(data: &[u8]) -> String {
    let reader = rexif::parse_buffer(data).unwrap();
    let mut result = String::new();
    for entry in &reader.entries {
        result.push_str(&entry.value_more_readable);
    }
    result
}