use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{
    config::Builder,
    types::{AttributeDefinition, BillingMode, KeySchemaElement, KeyType, ScalarAttributeType},
    Client, Error,
};
use lib::strikes_db::{delete_all_strikes, get_strikes, increment_strikes, StrikeEntity};
use uuid::Uuid;

async fn create_random_table(client: &Client) -> Result<String, Error> {
    let random_table_name = format!("Strikes_{}", Uuid::new_v4());
    let pk = AttributeDefinition::builder()
        .attribute_name("UserId")
        .attribute_type(ScalarAttributeType::S)
        .build()?;

    let ks = KeySchemaElement::builder()
        .attribute_name("UserId")
        .key_type(KeyType::Hash)
        .build()?;

    client
        .create_table()
        .table_name(&random_table_name)
        .key_schema(ks)
        .attribute_definitions(pk)
        .billing_mode(BillingMode::PayPerRequest)
        .send()
        .await?;

    Ok(random_table_name)
}

#[tokio::test]
async fn it_should_add_some_strikes() -> Result<(), Box<dyn std::error::Error>> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let local_config = Builder::from(&config)
        .endpoint_url("http://localhost:8000")
        .build();
    let client = Client::from_conf(local_config);

    let table_name = create_random_table(&client).await.unwrap();

    let _ = increment_strikes("heinz", &table_name, &client)
        .await
        .unwrap();
    let _ = increment_strikes("heinz", &table_name, &client)
        .await
        .unwrap();
    let strikes = increment_strikes("heinz", &table_name, &client)
        .await
        .unwrap();

    assert_eq!(strikes, 3);

    Ok(())
}

#[tokio::test]
async fn it_should_get_a_list_of_strikes() -> Result<(), Box<dyn std::error::Error>> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let local_config = Builder::from(&config)
        .endpoint_url("http://localhost:8000")
        .build();
    let client = Client::from_conf(local_config);

    let table_name = create_random_table(&client).await.unwrap();

    let _ = increment_strikes("heinz", &table_name, &client)
        .await
        .unwrap();
    let _ = increment_strikes("heinz", &table_name, &client)
        .await
        .unwrap();
    let _ = increment_strikes("heinz", &table_name, &client)
        .await
        .unwrap();
    let _ = increment_strikes("guenther", &table_name, &client)
        .await
        .unwrap();

    let strikes = get_strikes(&table_name, &client).await.unwrap();

    assert_eq!(
        strikes,
        vec![
            StrikeEntity {
                user_id: "heinz".to_string(),
                strikes: 3
            },
            StrikeEntity {
                user_id: "guenther".to_string(),
                strikes: 1
            }
        ]
    );

    Ok(())
}

#[tokio::test]
async fn it_should_delete_all_items() -> Result<(), Box<dyn std::error::Error>> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let local_config = Builder::from(&config)
        .endpoint_url("http://localhost:8000")
        .build();
    let client = Client::from_conf(local_config);

    let table_name = create_random_table(&client).await.unwrap();

    let _ = increment_strikes("heinz", &table_name, &client)
        .await
        .unwrap();
    let _ = increment_strikes("heinz", &table_name, &client)
        .await
        .unwrap();
    let _ = increment_strikes("heinz", &table_name, &client)
        .await
        .unwrap();
    let _ = increment_strikes("guenther", &table_name, &client)
        .await
        .unwrap();

    delete_all_strikes(&table_name, &client).await.unwrap();
    let strikes = get_strikes(&table_name, &client).await.unwrap();

    assert_eq!(strikes, vec![]);

    Ok(())
}
