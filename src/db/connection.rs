use elasticsearch::auth::Credentials;
use elasticsearch::Elasticsearch;
use elasticsearch::http::transport::Transport;
use crate::cross_cutting::EnvironmentVariables;
use crate::domain::Error;

#[derive(Clone)]
pub struct MongoDbClient {
    database_url: String,
    db_name: String,
}

pub struct MongoClient;

impl MongoClient {
    pub async fn conn() -> mongodb::Database {
        let connection_string = EnvironmentVariables::get_mongo_connection_string().unwrap();
        let db_name = EnvironmentVariables::get_mongo_db_name().unwrap();

        let client = mongodb::Client::with_uri_str(&connection_string).await;
        let db = client.unwrap().database(&db_name);

        db
    }

    pub async fn get_collection<T>(collection_name: &str) -> mongodb::Collection<T> {
        let db = MongoClient::conn().await;

        db.collection::<T>(collection_name)
    }
}
pub struct ElasticsearchClient;

impl ElasticsearchClient {
    pub async fn conn() -> Result<Elasticsearch, Error> {
        let cloud_id = "My_deployment:ZXVyb3BlLXdlc3Q0LmdjcC5lbGFzdGljLWNsb3VkLmNvbSRjMmMxNTJkYjg4NGY0NmZhYjgxYzU4MDhkYjBkM2NmYSRhZGNmYjRkMzViNGQ0ZmE0ODgwMGM4NWEzYTBhZjZmYQ==";
        // can use other types of Credentials too, like Bearer or ApiKey
        let credentials = Credentials::Basic("elastic".into(), "higFbZGkTmacLKkByimvG8Yo".into());
        let transport = Transport::cloud(cloud_id, credentials)
            .map_err(|e| Error::internal_server_error(e.to_string()))?;
        Ok(Elasticsearch::new(transport))
    }
}
