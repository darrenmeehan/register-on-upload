#[macro_use]
extern crate log;
extern crate simple_logger;
use lambda::lambda;
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, PutItemInput};
use serde_json::Value;
use std::collections::HashMap;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[lambda]
#[tokio::main]
async fn main(event: Value) -> Result<Value, Error> {
    let logger = simple_logger::init();
    match logger {
        Ok(logger) => debug!("Logger has been setup successfully"),
        Err(error) => println!("Something went wrong"),
    };
    debug!("event={}", event);

    let table_name = get_dynamodb_table_name();

    // FIXME Add support for more than one record..
    let image_information = get_image_information(&event);
    let id = event["Records"][0]["s3"]["object"]["key"].to_string();
    debug!("id={}", id);

    let details = AttributeValue {
        s: Option::from(image_information),
        ..Default::default()
    };
    let id_value = AttributeValue {
        s: Option::from(id),
        ..Default::default()
    };

    // FIXME Hardcoded value for now is ok as currently all photos are uploaded by me
    // Adding stubbed value to enable API queries required by UI..
    let owner = AttributeValue {
        s: Option::from("darren.meehan".to_string()),
        ..Default::default()
    };

    let mut item: HashMap<String, AttributeValue> = HashMap::new();
    item.insert("id".to_string(), id_value);
    item.insert("Details".to_string(), details);
    item.insert("OwnedBy".to_string(), owner);
    let input = PutItemInput {
        item,
        table_name,
        ..Default::default()
    };
    let client = DynamoDbClient::new(Region::EuWest1);
    let response = client.put_item(input).await;

    // FIXME Return response
    let response = match response {
        Ok(entry) => debug!("Added entry"), // FIXME Figure out how to log entry
        // FIXME Fail Lambda run on error
        Err(error) => panic!("Problem adding to DB: {}", error),
    };
    Ok(event)
}

fn get_dynamodb_table_name() -> String {
// FIXME Get table name from environment variable
    let table_name: String = "photos".to_string();
    table_name
}

fn get_image_information(event: &Value) -> String {
    let s3_object = event["Records"][0]["s3"]["object"].to_string();
    s3_object
}
