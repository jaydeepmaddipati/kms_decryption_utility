#[inline]
pub fn is_valid_base64_encoded<T: AsRef<[u8]>>(encoded_string: T) -> bool {
    matches!(base64::decode(encoded_string), Ok(_))
}
