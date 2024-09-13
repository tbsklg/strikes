use ::serde::Serialize;
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{types::AttributeValue, Client};
use lambda_http::{
    aws_lambda_events::apigw::ApiGatewayWebsocketProxyRequest,
    lambda_runtime::{self},
    tracing,
};
use lambda_runtime::{service_fn, Error, LambdaEvent};

#[derive(Debug, Serialize)]
struct Response {
    #[serde(rename = "statusCode")]
    status_code: i32,
}

async fn function_handler(
    event: LambdaEvent<ApiGatewayWebsocketProxyRequest>,
) -> Result<Response, Error> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);

    add_connection_id(event, &client, "Connections").await
}

async fn add_connection_id(
    event: LambdaEvent<ApiGatewayWebsocketProxyRequest>,
    client: &Client,
    table_name: &str,
) -> Result<Response, Error> {
    client
        .put_item()
        .table_name(table_name)
        .item(
            "ConnectionId",
            AttributeValue::S(event.payload.request_context.connection_id.unwrap()),
        )
        .send()
        .await?;

    Ok(Response { status_code: 200 })
}

#[tokio::main]
#[allow(dead_code)]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    lambda_runtime::run(service_fn(function_handler)).await?;
    Ok(())
}

#[cfg(test)]
mod integration_tests {
    use aws_config::BehaviorVersion;
    use aws_lambda_events::apigw::{
        ApiGatewayWebsocketProxyRequest, ApiGatewayWebsocketProxyRequestContext,
    };
    use aws_sdk_dynamodb::{
        config::Builder,
        types::{AttributeDefinition, BillingMode, KeySchemaElement, KeyType, ScalarAttributeType},
        Client, Error,
    };
    use lambda_http::{lambda_runtime, Context};
    use lambda_runtime::LambdaEvent;
    use uuid::Uuid;

    use crate::add_connection_id;

    async fn create_random_table(client: &Client) -> Result<String, Error> {
        let random_table_name = format!("Connections_{}", Uuid::new_v4());
        let pk = AttributeDefinition::builder()
            .attribute_name("ConnectionId")
            .attribute_type(ScalarAttributeType::S)
            .build()?;

        let ks = KeySchemaElement::builder()
            .attribute_name("ConnectionId")
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
    async fn should_register_connection_id() {
        let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let local_config = Builder::from(&config)
            .endpoint_url("http://localhost:8000")
            .build();

        let client = Client::from_conf(local_config);
        let table_name = create_random_table(&client).await.unwrap();

        let request_context = ApiGatewayWebsocketProxyRequestContext {
            connection_id: Some("abcdefghijkl".to_string()),
            ..Default::default()
        };

        let proxy_event = ApiGatewayWebsocketProxyRequest {
            request_context,
            ..Default::default()
        };

        let context = Context::default();
        let event = LambdaEvent {
            payload: proxy_event,
            context,
        };

        let response = add_connection_id(event, &client, &table_name)
            .await
            .unwrap();

        let connection_ids = client
            .scan()
            .table_name(&table_name)
            .send()
            .await
            .unwrap()
            .items
            .unwrap();

        assert_eq!(response.status_code, 200);
        assert_eq!(connection_ids.len(), 1);
    }
}
