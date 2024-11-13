use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use url::Url;

use crate::clients::{CacheApiType, ProductApiType, TaskTrackerApiType};

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    // cover database connection
    pub db_uri: Url,
    pub db_pass_path: PathBuf,
    // port on which the cover API will listen for incoming connections
    pub listen_port: u16,
    // cover database connection
    pub product_api_uri: Url,
    pub product_api_pass_path: PathBuf,
    pub product_api_type: ProductApiType,
    // to update the cache when data is updated in db
    pub cache_api_uri: Url,
    pub cache_api_pass_path: PathBuf,
    pub cache_api_type: CacheApiType,
    // to return a uri of a task to track instead of waiting the operation.
    pub tasks_api_uri: Url,
    pub tasks_api_pass_path: PathBuf,
    pub tasks_api_type: TaskTrackerApiType,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            db_uri: Url::parse("postgresql:://user@127.0.0.1:5432/mydb").unwrap(),
            db_pass_path: PathBuf::from("name_api/db/user"),
            listen_port: 10200,
            product_api_uri: Url::parse("https://ecommerce.com/product-api").unwrap(),
            product_api_pass_path: PathBuf::from("product-api/user"),
            product_api_type: ProductApiType::Dolibarr("dolibarr/api/token".into()),
            cache_api_uri: Url::parse("https://ecommerce.com/cache-api").unwrap(),
            cache_api_pass_path: PathBuf::from("cache-api/user"),
            cache_api_type: CacheApiType::Mnemosyne,
            tasks_api_uri: Url::parse("https://ecommerce.com/tasks-api").unwrap(),
            tasks_api_pass_path: PathBuf::from("tasks-api/user"),
            tasks_api_type: TaskTrackerApiType::TaskTrackerRs("tasks-api/admin".into()),
        }
    }
}
