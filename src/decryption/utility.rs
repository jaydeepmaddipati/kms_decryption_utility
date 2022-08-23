use super::aws_kms_helper::*;
use super::encrypted_cipher_text::*;
use crate::datasource::internal::*;
use futures::future::join_all;
use std::collections::HashMap;

use super::aes_helper::get_decrypted_cipher_text;
use crate::DataSource;
use time::Instant;
use tokio::sync::mpsc::UnboundedSender;
use tracing::log::{debug, info};

pub async fn decrypt_data(
    mut datasource: impl DataSource,
    tx: UnboundedSender<HashMap<String, String>>,
) {
    let row_data_iter = match datasource.load_and_check_data() {
        Ok(row_data_iter) => row_data_iter,
        Err(error) => {
            panic!("Error occurred during check data step! {:?}", error)
        }
    };
    let kms_client = get_kms_client().await;
    let mut futures_vec = vec![];
    let start = Instant::now();
    row_data_iter.for_each(|row_data| {
        debug!("decrypting data...");
        match row_data {
            RowData::Generic(_) => {}
            RowData::Decryptable(mut key_values) => {
                let kms_client = kms_client.clone();
                let tx = tx.clone();
                let decryption_task_handle = tokio::spawn(async move {
                    debug!("running new decryption task");
                    let encryption_key = key_values.get(ENCRYPTION_KEY).unwrap();
                    let encryption_context = key_values.get(ENCRYPTION_CONTEXT).unwrap();
                    let plain_text_key =
                        get_plain_text_key(kms_client, encryption_key, encryption_context)
                            .await
                            .unwrap();
                    let plain_text_key_bytes_ref = plain_text_key.as_ref();
                    for (key, value) in &mut key_values {
                        if key != ENCRYPTION_KEY
                            && key != ENCRYPTION_CONTEXT
                            && is_valid_base64_encoded(&value)
                            && is_valid_base64_encoded(base64::decode(&value).unwrap())
                        {
                            let encrypted_cipher_text = get_encrypted_cipher_text(&value);
                            let decrypted_ciphertext: String = get_decrypted_cipher_text(
                                plain_text_key_bytes_ref,
                                encrypted_cipher_text,
                            )
                            .unwrap();
                            *value = decrypted_ciphertext;
                        }
                    }
                    key_values.remove(ENCRYPTION_KEY);
                    key_values.remove(ENCRYPTION_CONTEXT);
                    tx.send(key_values).expect("could not send message");
                });
                futures_vec.push(decryption_task_handle);
            }
            RowData::NonDecryptable(_) => {}
        }
    });
    join_all(futures_vec).await;
    info!(
        "time taken to complete all futures = {} secs",
        start.elapsed().whole_seconds()
    );
}
