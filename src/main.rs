pub mod database_handler;
pub mod url_data;

use std::env;
use dotenv::dotenv;
use mongodb::error;

use database_handler::UrlCollection;


fn get_env_vars(var_name: &str) -> String {
	env::var(var_name).expect("{var_name} must be set")
}

#[tokio::main]
async fn main() -> error::Result<()> {

	dotenv().ok();
	let uri = get_env_vars("MONGO_URI");
	let db_name = get_env_vars("MONGO_DB_NAME");
	let collection_name = get_env_vars("MONGO_COLLECTION_NAME");

	let x = UrlCollection::new(uri.as_str(), db_name.as_str(), collection_name.as_str()).await?;

	if let Some(mut y) = x.get_url_data(2).await? {
		println!("{:?}", y);
		y.original_url = "youtube.com".to_string();
		x.modify_url_data(2, &y).await?;
		println!("{:?}", y);
	}

	Ok(())
}

