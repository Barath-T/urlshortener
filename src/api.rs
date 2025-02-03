use std::collections::HashMap;

use crate::database_handler::UrlCollection;
use crate::request::Request;
use crate::url_data::UrlData;

use base62;

pub async fn handle_get(request_object: &mut Request, db: &UrlCollection) -> Result<(), &'static str> {
    let body = match &request_object.body {
        Some(b) => b,
        None => {
            request_object.response(
                400,
                &HashMap::from([("message", "Bad Request: Recieved empty body")]),
            );

            return Ok(());
        }
    };
    let shortened_url = match body.get("short_url") {
        Some(v) => v,
        None => "^()^#",
    };
    if shortened_url == "^()^#" {
        return Err("No shortened url found");
    }
    let short_url = base62::decode(shortened_url).unwrap() as i64;
    let mut data = db
        .get_url_data(short_url)
        .await
        .expect("Cannot get data")
        .or(Err("No records found")?)
        .unwrap();
    let original_url = data.original_url.clone();
    match data.uses_left {
        Some(v) => {
            if v == 1 {
                db.delete_url_data(short_url);
            } else {
                data.uses_left = Some(data.uses_left.unwrap() - 1);
                db.modify_url_data(short_url, data);
            }
        }
        None => {}
    }

    match request_object.response(200, &HashMap::from([("original_url", original_url)])) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("{:?}", err);
            return Err("Couldn't send response");
        }
    };
    Ok(())
}

pub async fn handle_post(request_object: &mut Request, db: &UrlCollection) -> Result<(), &'static str> {
    let body = match &request_object.body {
        Some(b) => b,
        None => {
            request_object.response(
                400,
                &HashMap::from([("message", "Bad Request: Recieved empty body")]),
            );

            return Ok(());
        }
    };
    let new_id = match db.get_new_id().await {
        Ok(v) => v + 1,
        _ => Err("Cannot Get Last Id or create one")?,
    };
    let short_url = base62::encode(new_id);
    let original_url = match body.get("original_url") {
        Some(v) => v.to_owned(),
        None => Err("Original URL not found in request body")?,
    };
    let expiration_date = match body.get("expiration_date") {
        Some(v) => v.to_owned(),
        None => Err("Expiration Date not found in request body")?,
    };
    let uses_left = match body.get("uses_left") {
        Some(v) => Some(v.parse::<i32>().unwrap()),
        None => None,
    };
    match match db
        .insert_url_data(UrlData {
            short_url: short_url.clone(),
            original_url: original_url,
            uses_left: uses_left,
            expiration_date: expiration_date,
        })
        .await
    {
        Ok(_) => request_object.response(
            200,
            &HashMap::from([("message", "success"), ("short_url", short_url.as_str())]),
        ),
        Err(err) => {
            request_object.response(500, &HashMap::from([("message", "internal server error")]))
        }
    } {
        Ok(_) => (),
        Err(err) => {
            eprintln!("{:?}", err);
            return Err("Couldn't send response");
        }
    }
    Ok(())
}
