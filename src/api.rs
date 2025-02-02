use std::collections::HashMap;

use crate::database_handler::UrlCollection;
use crate::url_data::UrlData;

use base62;


pub struct RequestObject {
    header: HashMap<String, String>,
    body: HashMap<String,String>,
}



pub async fn handle_get(request_object: &mut RequestObject, db: UrlCollection) -> Result< String,&str> {

    let shortened_url = match request_object.body.get("short_url") {
        Some(v) => v,
        None => "^()^#",
    };
    if shortened_url == "^()^#" {
        return Err("No shortened url found");
    }
    let short_url = base62::decode(shortened_url).unwrap() as i64;
    let mut data = db.get_url_data(short_url).
                                    await.expect("Cannot get data").
                                    or(Err("No records found")?).unwrap();
    let original_url = data.original_url.clone();
    match data.uses_left {
        Some(v) => if v==1 { db.delete_url_data(short_url);} else { data.uses_left= Some(data.uses_left.unwrap() - 1);db.modify_url_data(short_url, data );} 
        None => {}
    }
    
    Ok(original_url)

}

pub async fn handle_post(request_object: &mut RequestObject, db: UrlCollection) -> Result<(), &str> {
    let new_id = match db.get_new_id().await {
        Ok(v) => v + 1,
        _ =>  Err("Cannot Get Last Id or create one")?,
    };
    let short_url = new_id;
    let original_url = match request_object.body.get("original_url") {
        Some(v) => v.to_owned(),
        None => Err("Original URL not found in request body")?,
    };
    let expiration_date = match request_object.body.get("expiration_date") {
        Some(v) => v.to_owned(),
        None => Err("Expiration Date not found in request body")?,
    };
    let uses_left = match request_object.body.get("uses_left") {
        Some(v) => Some(v.parse::<i32>().unwrap()),
        None => None,
    };
    db.insert_url_data(UrlData { 
        short_url: short_url, 
        original_url: original_url, 
        uses_left: uses_left,
        expiration_date: expiration_date, 
    }).await.expect("Cannot insert record");
    Ok(())

}
