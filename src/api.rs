use base62;

struct Data {
    id: u64,
    shortened_url: String,
    original_url: String,
    no_of_uses: u64,
    expiration_date: String,
    max_no_of_uses: u64

}

struct RequestObject {
    header: HashMap<String, String>,
    body: HashMap<String,String>,
}

pub fn get_record(shortened_url: &str) -> Result<Data,String> {
    Err("Not Implemented".to_owned())
}

pub fn delete_record(shortened_url: &str) -> Result<(),String> {
    Err("Not Implemented".to_owned())
}

pub fn modify_record(shortened_url: &str, data: Data) -> Result<(),String>  {
    Err("Not Implemented".to_owned())
}

pub fn insert_record(data: Data) -> Result<(),String> {
    Err("Not Implemented".to_owned())
}

pub fn get_last_id() -> Result<u64, String> {
    Err("Not Implemented".to_owned())
}

pub fn handle_get(request_object: &mut RequestObject) -> Result< String,&str> {

    let shortened_url = match request_object.body.get("shorteneed_url") {
        Some(v) => v,
        None => "^()^#",
    };
    if shortened_url == "^()^#" {
        return Err("No shortened url found");
    }
    let mut data = get_record(shortened_url).expect("No Record found");
    if data.no_of_uses == data.max_no_of_uses {
        delete_record(shortened_url).expect("Cannot Delete Record");
    }
    else {
        data.no_of_uses = data.no_of_uses + 1;
        modify_record(shortened_url, data).expect("Cannot Modify Record");
    }
    Ok(shortened_url.to_owned())

}

pub fn handle_post(request_object: &mut RequestObject) -> Result<(), &str> {
    let new_id = match get_last_id() {
        Ok(v) => v + 1,
        Err(e) => if e ==  "No Records".to_string() {
                100000000000_u64
            }
            else {
                Err("Cannot Get Last Id or create one")?
            },
    };
    let shortened_url = base62::encode(new_id);
    let original_url = match request_object.body.get("original_url") {
        Some(v) => v.to_owned(),
        None => Err("Original URL not found in request body")?,
    };
    let expiration_date = match request_object.body.get("expiration_date") {
        Some(v) => v.to_owned(),
        None => Err("Expiration Date not found in request body")?,
    };
    let max_no_of_uses = match request_object.body.get("max_no_of_uses") {
        Some(v) => v.parse::<u64>().unwrap(),
        None => Err("Maximum number of uses not found in request body")?,
    };
    insert_record(Data { 
        id: new_id, 
        shortened_url: shortened_url, 
        original_url: original_url, 
        no_of_uses: 0, 
        expiration_date: expiration_date, 
        max_no_of_uses: max_no_of_uses  
    }).expect("Cannot insert record");
    Ok(())

}