use super::row::*;
use super::utility::*;
use super::{ENCRYPTION_CONTEXT, ENCRYPTION_KEY};
use std::collections::HashMap;
use tracing::log::debug;

pub type RowData = Row<HashMap<String, String>>;

pub trait FetchData {
    fn fetch_data(
        &mut self,
    ) -> Result<Box<dyn Iterator<Item = RowData> + '_>, Box<dyn std::error::Error>>;
}

pub trait CheckData: FetchData {
    fn load_and_check_data(
        &mut self,
    ) -> Result<Box<dyn Iterator<Item = RowData> + '_>, Box<dyn std::error::Error>> {
        let fetch_data_iter = match self.fetch_data() {
            Ok(fetch_data_iter) => fetch_data_iter,
            Err(error) => panic!("Error occurred during processing row data {:?}", error),
        };
        let check_data_iter = Box::new(fetch_data_iter.map(|row| {
            debug!("validating data...");
            match row {
                RowData::Generic(row_data) => {
                    if row_data.contains_key(ENCRYPTION_KEY)
                        && row_data.contains_key(ENCRYPTION_CONTEXT)
                    {
                        if is_valid_base64_encoded(row_data.get(ENCRYPTION_KEY).unwrap()) {
                            Row::Decryptable(row_data)
                        } else {
                            Row::NonDecryptable(row_data)
                        }
                    } else {
                        Row::NonDecryptable(row_data)
                    }
                }
                _ => row,
            }
        }));
        Ok(check_data_iter)
    }
}

pub trait DataSource: CheckData {}
