use aws_sdk_dynamodb::{
    error::ProvideErrorMetadata,
    operation::scan::ScanOutput,
    types::{AttributeValue, ReturnValue},
    Client,
};
use lambda_http::Error;
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StrikeEntity {
    pub user_id: String,
    pub strikes: u8,
}

pub async fn delete_all_strikes(table_name: &str, client: &Client) -> Result<(), Error> {
    let strikes = get_strikes(table_name, client).await?;

    for strike in strikes {
        client
            .delete_item()
            .table_name(table_name)
            .key("UserId", AttributeValue::S(strike.user_id))
            .send()
            .await?;
    }

    Ok(())
}

pub async fn get_strikes(table_name: &str, client: &Client) -> Result<Vec<StrikeEntity>, Error> {
    let request: ScanOutput = client.scan().table_name(table_name).send().await?;

    request
        .items()
        .iter()
        .map(|item| {
            let user_id = item.get("UserId").unwrap().as_s().unwrap().to_string();
            let strikes = item
                .get("Strikes")
                .unwrap()
                .as_n()
                .unwrap()
                .parse()
                .unwrap();

            Ok(StrikeEntity { user_id, strikes })
        })
        .collect()
}

pub async fn increment_strikes(
    username: &str,
    table_name: &str,
    client: &Client,
) -> Result<u8, Error> {
    let request = client
        .update_item()
        .table_name(table_name.to_string())
        .key("UserId", AttributeValue::S(username.to_string()))
        .update_expression("set Strikes = Strikes + :value")
        .expression_attribute_values(":value", AttributeValue::N("1".to_string()))
        .return_values(ReturnValue::UpdatedNew)
        .send()
        .await
        .map_err(|err| err.into_service_error());

    match request {
        Ok(response) => {
            let strike_count = extract_strike_count(response.attributes().unwrap());
            Ok(strike_count)
        }
        Err(err) => match ProvideErrorMetadata::code(&err) {
            Some("ValidationException") => add_user(username, table_name, client).await,
            _ => Err(err.into()),
        },
    }
}

async fn add_user(username: &str, table_name: &str, client: &Client) -> Result<u8, Error> {
    client
        .put_item()
        .table_name(table_name.to_string())
        .item("UserId", AttributeValue::S(username.to_string()))
        .item("Strikes", AttributeValue::N("1".to_string()))
        .send()
        .await?;

    Ok(1)
}

fn extract_strike_count(map: &HashMap<String, AttributeValue>) -> u8 {
    map.get("Strikes").unwrap().as_n().unwrap().parse().unwrap()
}
