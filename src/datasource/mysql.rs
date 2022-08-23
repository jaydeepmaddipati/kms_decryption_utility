use std::collections::HashMap;
use std::error::Error;

use mysql::prelude::*;
use mysql::*;
use tracing::log::{debug, error, info};

use crate::DataSourceBuilder;
use crate::Row::Generic;

use super::base::*;

pub struct MySQLDataSource {
    pub connection: Conn,
    pub query_string: String,
}

impl MySQLDataSource {
    pub fn new(datasource_builder: DataSourceBuilder) -> std::result::Result<Self, Box<dyn Error>> {
        Self::load_from_config(datasource_builder)
    }

    fn load_from_config(
        datasource_builder: DataSourceBuilder,
    ) -> std::result::Result<Self, Box<dyn Error>> {
        info!("started loading datasource from config");
        let mysql_user = datasource_builder
            .get_config("MYSQL_USER")
            .expect("missing env var MYSQL_USER");
        let mysql_password = datasource_builder
            .get_config("MYSQL_PASSWORD")
            .expect("missing env var MYSQL_PASSWORD");
        let mysql_host_name = datasource_builder
            .get_config("MYSQL_HOST_NAME")
            .expect("missing env var MYSQL_HOST_NAME");
        let mysql_port = datasource_builder
            .get_config("MYSQL_PORT")
            .expect("missing env var MYSQL_PORT");
        let mysql_database = datasource_builder
            .get_config("MYSQL_DATABASE")
            .expect("missing env var MYSQL_DATABASE");
        let query_string = datasource_builder
            .get_config("query")
            .expect("missing config param query")
            .to_string();

        let url = format!(
            "mysql://{}:{}@{}:{}/{}",
            mysql_user, mysql_password, mysql_host_name, mysql_port, mysql_database
        );
        let opts = Opts::from_url(&url)
            .expect("datasource connection string/url should be in proper format");
        let connection = match Conn::new(opts) {
            Ok(connection) => {
                info!("successfully created connection with mysql datasource");
                connection
            }
            Err(error) => {
                error!("Problem creating new datasource connection : {:?}", error);
                panic!()
            }
        };
        Ok(MySQLDataSource {
            connection,
            query_string,
        })
    }
}

impl FetchData for MySQLDataSource {
    fn fetch_data(
        &mut self,
    ) -> std::result::Result<Box<dyn Iterator<Item = RowData> + '_>, Box<dyn Error>> {
        let query_iter = match self.connection.query_iter(&self.query_string) {
            Ok(query_iter) => query_iter,
            Err(error) => {
                error!("Problem fetching data!: {:?}", error);
                panic!()
            }
        };
        let query_iter = query_iter.map(|row_data_result| {
            debug!("processing data into required schema");
            let row_data = row_data_result.unwrap();
            let columns = row_data.columns();
            let values = row_data.unwrap();
            let mut key_values = HashMap::new();
            for (value, column) in values.iter().zip(columns.iter()) {
                if *value == Value::NULL {
                    continue;
                }
                key_values.insert(
                    column.name_str().to_string(),
                    from_value::<String>(value.to_value()),
                );
            }
            Generic(key_values)
        });
        Ok(Box::new(query_iter))
    }
}

impl CheckData for MySQLDataSource {}

impl DataSource for MySQLDataSource {}
