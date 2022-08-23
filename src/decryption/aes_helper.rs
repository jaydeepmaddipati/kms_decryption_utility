use std::error::Error;

use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, KeyIvInit};
use tracing::log::{debug, error};

use super::encrypted_cipher_text::EncryptedCipherText;

type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

#[inline]
pub(crate) fn get_decrypted_cipher_text(
    plain_text_key_bytes: &[u8],
    encrypted_cipher_text: EncryptedCipherText,
) -> Result<String, Box<dyn Error>> {
    debug!("decrypting cipher text");
    let decryptor = Aes256CbcDec::new_from_slices(
        plain_text_key_bytes,
        &base64::decode(encrypted_cipher_text.iv).unwrap(),
    )
    .expect("invalid plain text key");
    let decrypted_ciphertext = match decryptor
        .decrypt_padded_vec_mut::<Pkcs7>(&base64::decode(encrypted_cipher_text.value).unwrap())
    {
        Ok(decrypted_ciphertext) => decrypted_ciphertext,
        Err(error) => {
            error!("Error decrypting cipher text : {:?}", error);
            panic!()
        }
    };
    let decrypted_ciphertext = String::from_utf8(decrypted_ciphertext).unwrap();
    let decrypted_ciphertext: Vec<&str> = decrypted_ciphertext.split(':').collect();
    let decrypted_ciphertext = decrypted_ciphertext.last().unwrap();
    let decrypted_ciphertext = &decrypted_ciphertext[1..decrypted_ciphertext.len() - 2];
    Ok(decrypted_ciphertext.to_string())
}
