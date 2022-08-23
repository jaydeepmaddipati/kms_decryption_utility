use std::error::Error;

use aws_sdk_kms::client::Client;
use aws_sdk_kms::types::Blob;
use tracing::log::error;

pub(crate) async fn get_kms_client() -> Client {
    let shared_config = aws_config::load_from_env().await;
    Client::new(&shared_config)
}

pub(crate) async fn get_plain_text_key(
    kms_client: Client,
    encryption_key: &String,
    encryption_context: &String,
) -> Result<Blob, Box<dyn Error>> {
    let encrypted_key = base64::decode(encryption_key).expect(
        "base64 decode of encryption should not fail here, as it is already validated prior",
    );
    let kms_decryption_output = match kms_client
        .decrypt()
        .encryption_context("id", encryption_context)
        .ciphertext_blob(Blob::new(encrypted_key))
        .send()
        .await
    {
        Ok(kms_decryption_output) => kms_decryption_output,
        Err(error) => {
            error!("decryption of encrypted key failed {:?}", error);
            panic!()
        }
    };
    // plain text key cannot be none here due to data invariant
    Ok(kms_decryption_output.plaintext.unwrap())
}
