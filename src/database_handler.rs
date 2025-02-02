use crate::url_data::UrlData;
use mongodb::{Client, Database, Collection, error, options::ReturnDocument, bson::{Document, doc}};

pub struct UrlCollection {
	pub client: Client,
	pub database: Database,
	pub collection: Collection<UrlData>
}

impl UrlCollection {

	pub async fn new(uri: &str, database_name: &str, collection_name: &str) -> error::Result<Self> {
		let client = Client::with_uri_str(uri).await?;
		let database = client.database(database_name);
		let collection = database.collection(collection_name);

		Ok(UrlCollection {client, database, collection})
	}

	pub async fn get_url_data(&self, short_url: i64) -> error::Result<Option<UrlData>> {
		self.collection.find_one(doc! {"_id": short_url }).await
	}

	pub async fn insert_url_data(&self, url_data: UrlData) -> error::Result<()> {
		self.collection.insert_one(url_data).await?;
		Ok(())
	}

	pub async fn modify_url_data(&self, short_url: i64, new_url_data: UrlData) -> error::Result<()> {
		self.collection.update_one(
			doc! {"_id": short_url},
			doc! {
				"$set": {
					"original_url": new_url_data.original_url,
					"uses_left": new_url_data.uses_left,
					"expiration_date": new_url_data.expiration_date
				}
 			}
 		).await?;
		Ok(())
	}

	pub async fn delete_url_data(&self, short_url: i64) -> error::Result<()> {
		self.collection.delete_one(doc! {"_id": short_url}).await?;
		Ok(())
	}

	pub async fn get_new_id(&self) -> error::Result<u64> {
		let meta = self.database.collection::<Document>("meta");
		let update_results = meta.find_one_and_update(
			doc! {"_id": "COUNTER"},
			doc! {
				"$inc": doc! {"value": 1}
			},
		).return_document(ReturnDocument::After)
		.await?;

		if let Some(counter) = update_results {
			let new_id = counter.get_i64("value").expect("value field not set in _id with \"COUNTER\"");
			Ok(new_id as u64)
		} else {
			Err(error::Error::custom(
				"_id with \"COUNTER\" not found in collection {self.database}.{meta}".to_string()
			))
		}
	}
}

