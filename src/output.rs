use std::error::Error;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::io::BufWriter;
use tracing::log::{debug, error, warn};

#[inline]
pub async fn create_file_writer(file_name: &str) -> Result<BufWriter<File>, Box<dyn Error>> {
    let file = match File::create(file_name).await {
        Ok(file) => file,
        Err(error) => {
            error!(
                "could not create file {:?} with error = {:?}",
                file_name, error
            );
            panic!()
        }
    };
    let buf_writer = BufWriter::new(file);
    Ok(buf_writer)
}

#[inline]
pub async fn write_contents_to_file<T: AsRef<[u8]>>(buf_writer: &mut BufWriter<File>, contents: T) {
    match buf_writer.write_all(contents.as_ref()).await {
        Ok(_) => debug!("writing contents to file"),
        Err(error) => warn!("could not write contents to file with error = {:?}", error),
    }
}
