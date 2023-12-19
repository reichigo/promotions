use std::env::{self, VarError};
pub struct EnvironmentVariables;

impl EnvironmentVariables {

    //mongo
    pub fn set_mongo_connection_string(connection_string: &str) {
        env::set_var("MongoConnectionString", connection_string);
    }

    pub fn get_mongo_connection_string() -> Result<String, VarError> {
        env::var("MongoConnectionString")
    }

    pub fn set_mongo_db_name(mongo_db_name: &str) {
        env::set_var("MongoDbName", mongo_db_name);
    }

    pub fn get_mongo_db_name() -> Result<String, VarError> {
        env::var("MongoDbName")
    }
    //
}
