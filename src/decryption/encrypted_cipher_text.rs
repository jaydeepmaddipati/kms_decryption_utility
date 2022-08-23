use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct EncryptedCipherText {
    pub(crate) iv: String,
    pub(crate) value: String,
}
/// Returns base64 decoded `encrypted cipher text` along with initialization vector (random text)
/// can do explicit unwraps here as we have already checked for necessary conditions prior to
/// calling this function
#[inline]
pub(crate) fn get_encrypted_cipher_text<T: AsRef<[u8]>>(
    encrypted_cipher_text: T,
) -> EncryptedCipherText {
    let first_base_64_decoded_value = base64::decode(encrypted_cipher_text);
    let first_base_64_decoded_value = first_base_64_decoded_value.unwrap();
    let double_base_64_decoded_value =
        String::from_utf8(base64::decode(first_base_64_decoded_value).unwrap()).unwrap();
    serde_json::from_str(&double_base_64_decoded_value).unwrap()
}
