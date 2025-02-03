pub mod api;
pub mod database_handler;
pub mod url_data;
pub mod request;
pub mod server;

use std::env;
use dotenv::dotenv;
use mongodb::error;

use database_handler::UrlCollection;

use request::{Request, RequestType};
use server::Server;

use std::collections::HashMap;
use std::net::TcpStream;

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
  // Tests?
  // if let Some(mut y) = x.get_url_data(2).await? {
	//	 println!("{:?}", y);
	//	 y.original_url = "youtube.com".to_string();
	//	 x.modify_url_data(2, &y).await?;
	//	 println!("{:?}", y);
	//  }
  
  let server: Server = Server::new("127.0.0.1:8080");


  loop {
        let stream: TcpStream = server.accept();
        let mut request = match Request::new(stream) {
            Ok(req) => req,
            Err(err) => {
                eprintln!("{:?}", err);
                continue;
            }
        };
        println!("{:?}", request);

        match request.path.as_str() {
            "/" => match request.req_type {
                RequestType::POST => todo!(),
                RequestType::GET => todo!(),
            },
            _ => match request.response(404, &HashMap::from([("message", "not found")])) {
                Err(err) => {
                    eprintln!("{:?}", err);
                    continue;
                }
                _ => (),
            },
        }
    }

	Ok(())
}

