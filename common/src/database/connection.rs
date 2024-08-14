use std::time::Duration;

use mongodb::{options::{ClientOptions, Compressor, ReadPreference, ReadPreferenceOptions, SelectionCriteria, ServerApiVersion}, Client};

use crate::helpers::env::env::ENV;

pub async fn connect()-> Result<Client, mongodb::error::Error> {
    let database_string = ENV
        .get_string("DATABASE_URL")
        .expect("DATABASE_URL is missing");
let mut client_options = ClientOptions::parse(database_string).await?;
    // Establece la opción allowUseDisk en true
let read_preference = ReadPreference::Secondary {
    options: Some(ReadPreferenceOptions::builder().build()),
    };
    client_options.selection_criteria = Some(SelectionCriteria::ReadPreference(read_preference));
    client_options.default_database = Some("trs".to_string());
    client_options.app_name = Some("MyApp".to_string());
    client_options.server_api = Some(
        mongodb::options::ServerApi::builder()
            .version(ServerApiVersion::V1)
            .strict(true)
            .deprecation_errors(true)
            .build(),
    );
    client_options.retry_reads = Some(true);
    client_options.retry_writes = Some(true);
    client_options.max_pool_size = Some(8);
    client_options.read_concern = Some(mongodb::options::ReadConcern::local());
    client_options.write_concern = Some(mongodb::options::WriteConcern::majority());
    client_options.compressors = Some(vec![Compressor::Snappy]);
    client_options.local_threshold = Some(Duration::from_millis(15));
    client_options.heartbeat_freq = Some(Duration::from_secs(10));
    Client::with_options(client_options)
}
