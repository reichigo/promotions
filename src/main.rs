use std::path::PathBuf;
use crate::cross_cutting::EnvironmentVariables;
use crate::presentation::grpc::handlers::RegisterHandlers;
use crate::settings::Settings;

mod presentation;
mod domain;
mod db;
mod cross_cutting;
mod settings;
mod use_case;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::new(PathBuf::default()).expect("Failed to load configuration");
    //set EnvironmentVariables
    EnvironmentVariables::set_mongo_connection_string(&settings.mongodb.connection_string());
    EnvironmentVariables::set_mongo_db_name(&settings.mongodb.db_name);

    RegisterHandlers::register().await
}
