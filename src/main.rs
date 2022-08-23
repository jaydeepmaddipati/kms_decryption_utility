use std::collections::HashMap;

use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc;
use tracing::log::info;

use datasource::prelude::*;
use decryption::prelude::*;
use output::create_file_writer;
use output::write_contents_to_file;

use clap::Parser;

mod datasource;
mod decryption;
mod output;

#[derive(Parser, Debug)]
#[clap(name="aws_kms_decryption_application", author="Jaydeep Maddipati <jaydeepmaddipati@gmail.com>", 
version="0.1", about=None, long_about="application to decrypt data from a datasource with assumed data invariants.Check README.md for more details")]
struct Args {
    /// type of datasource
    #[clap(short, long, value_parser, default_value = "mysql")]
    datasource_type: String,

    /// valid datasource query to fetch data from
    #[clap(short, long, value_parser)]
    query: String,

    /// valid file path to persist decrypted data into
    #[clap(short, long, value_parser, default_value = "data.json")]
    path: String,
}

// #[tokio::main(flavor = "current_thread")]
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    tracing_subscriber::fmt::init();
    let datasource = DataSourceBuilder::new(&args.datasource_type)
        .set_config("query", &args.query)
        .load_from_env()?;
    let mut buf_writer = create_file_writer(&args.path).await?;
    let (tx, mut rx) = mpsc::unbounded_channel::<HashMap<String, String>>();
    decrypt_data(datasource, tx).await;
    while let Some(key_value) = rx.recv().await {
        write_contents_to_file(&mut buf_writer, serde_json::to_string(&key_value)?).await;
        write_contents_to_file(&mut buf_writer, b"\n").await;
    }
    info!("persisting decrypted data to json file {}", &args.path);
    buf_writer.flush().await?;
    info!("success!");
    Ok(())
}
