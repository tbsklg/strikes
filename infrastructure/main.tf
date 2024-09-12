terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.38.0"
    }
  }
  backend "s3" {
    bucket         = "tf-backend-state-strikes"
    encrypt        = true
    dynamodb_table = "tf-backend-lock-strikes"
    key            = "terraform.tfstate"
    region         = "eu-central-1"
  }
}

provider "aws" {
  region = "eu-central-1"
}

module "website" {
  source = "./website"
}

module "lambdas" {
  source = "./lambdas"
}

# -----------------------------------------------------------------------------
# REST API
# -----------------------------------------------------------------------------
resource "aws_api_gateway_rest_api" "strikes" {
  name = "strikes"
}

resource "aws_api_gateway_resource" "strikes" {
  parent_id   = aws_api_gateway_rest_api.strikes.root_resource_id
  path_part   = "strikes"
  rest_api_id = aws_api_gateway_rest_api.strikes.id
}

# -----------------------------------------------------------------------------
# GET STRIKES
# -----------------------------------------------------------------------------
resource "aws_api_gateway_method" "get_strikes" {
  authorization    = "NONE"
  http_method      = "GET"
  resource_id      = aws_api_gateway_resource.strikes.id
  rest_api_id      = aws_api_gateway_rest_api.strikes.id
  api_key_required = true
}

resource "aws_api_gateway_integration" "get_strikes" {
  http_method             = aws_api_gateway_method.get_strikes.http_method
  resource_id             = aws_api_gateway_resource.strikes.id
  rest_api_id             = aws_api_gateway_rest_api.strikes.id
  type                    = "AWS_PROXY"
  integration_http_method = "POST"
  uri                     = module.lambdas.get_strikes_lambda_invoke_arn
}

resource "aws_lambda_permission" "apigw_invoke_get_strikes_lambda" {
  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = module.lambdas.get_strikes_lambda_function_name
  principal     = "apigateway.amazonaws.com"

  source_arn = "${aws_api_gateway_rest_api.strikes.execution_arn}/*/*"
}

# -----------------------------------------------------------------------------
# DELETE STRIKES
# -----------------------------------------------------------------------------
resource "aws_api_gateway_method" "delete_strikes" {
  authorization    = "NONE"
  http_method      = "DELETE"
  resource_id      = aws_api_gateway_resource.strikes.id
  rest_api_id      = aws_api_gateway_rest_api.strikes.id
  api_key_required = true
}

resource "aws_api_gateway_integration" "delete_strikes" {
  http_method             = aws_api_gateway_method.delete_strikes.http_method
  resource_id             = aws_api_gateway_resource.strikes.id
  rest_api_id             = aws_api_gateway_rest_api.strikes.id
  type                    = "AWS_PROXY"
  integration_http_method = "POST"
  uri                     = module.lambdas.delete_strikes_lambda_invoke_arn
}

resource "aws_lambda_permission" "apigw_invoke_delete_strikes_lambda" {
  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = module.lambdas.delete_strikes_lambda_function_name
  principal     = "apigateway.amazonaws.com"

  source_arn = "${aws_api_gateway_rest_api.strikes.execution_arn}/*/*"
}

# -----------------------------------------------------------------------------
# PUT STRIKE
# -----------------------------------------------------------------------------
resource "aws_api_gateway_resource" "put_strike" {
  parent_id   = aws_api_gateway_resource.strikes.id
  path_part   = "{user}"
  rest_api_id = aws_api_gateway_rest_api.strikes.id
}

resource "aws_api_gateway_method" "put_strike" {
  authorization    = "NONE"
  http_method      = "PUT"
  resource_id      = aws_api_gateway_resource.put_strike.id
  rest_api_id      = aws_api_gateway_rest_api.strikes.id
  api_key_required = true
}

resource "aws_api_gateway_integration" "put_strike" {
  http_method             = aws_api_gateway_method.put_strike.http_method
  resource_id             = aws_api_gateway_resource.put_strike.id
  rest_api_id             = aws_api_gateway_rest_api.strikes.id
  type                    = "AWS_PROXY"
  integration_http_method = "POST"
  uri                     = module.lambdas.put_strike_lambda_invoke_arn
}

resource "aws_lambda_permission" "apigw_invoke_put_strike_lambda" {
  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = module.lambdas.put_strike_lambda_function_name
  principal     = "apigateway.amazonaws.com"

  source_arn = "${aws_api_gateway_rest_api.strikes.execution_arn}/*/*"
}

# -----------------------------------------------------------------------------
# HEALTH
# -----------------------------------------------------------------------------
resource "aws_api_gateway_resource" "health" {
  parent_id   = aws_api_gateway_rest_api.strikes.root_resource_id
  path_part   = "health"
  rest_api_id = aws_api_gateway_rest_api.strikes.id
}

resource "aws_api_gateway_method" "health" {
  authorization    = "NONE"
  http_method      = "GET"
  resource_id      = aws_api_gateway_resource.health.id
  rest_api_id      = aws_api_gateway_rest_api.strikes.id
  api_key_required = true
}

resource "aws_api_gateway_integration" "health" {
  http_method             = aws_api_gateway_method.health.http_method
  resource_id             = aws_api_gateway_resource.health.id
  rest_api_id             = aws_api_gateway_rest_api.strikes.id
  type                    = "AWS_PROXY"
  integration_http_method = "POST"
  uri                     = module.lambdas.health_lambda_invoke_arn
}

resource "aws_lambda_permission" "apigw_invoke_health_lambda" {
  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = module.lambdas.health_lambda_function_name
  principal     = "apigateway.amazonaws.com"

  source_arn = "${aws_api_gateway_rest_api.strikes.execution_arn}/*/*"
}

# -----------------------------------------------------------------------------
# DEPLOYMENT 
# -----------------------------------------------------------------------------
resource "aws_api_gateway_deployment" "strikes" {
  rest_api_id = aws_api_gateway_rest_api.strikes.id

  triggers = {
    redeployment = sha1(jsonencode([
      aws_api_gateway_resource.health.id,
      aws_api_gateway_method.health.id,
      aws_api_gateway_integration.health.id,
      aws_api_gateway_resource.put_strike.id,
      aws_api_gateway_method.put_strike.id,
      aws_api_gateway_integration.put_strike.id,
      aws_api_gateway_resource.strikes.id,
      aws_api_gateway_method.get_strikes.id,
      aws_api_gateway_integration.get_strikes.id,
      aws_api_gateway_method.delete_strikes.id,
      aws_api_gateway_integration.delete_strikes.id,
    ]))
  }

  lifecycle {
    create_before_destroy = true
  }
}

# -----------------------------------------------------------------------------
# STAGE 
# -----------------------------------------------------------------------------
resource "aws_api_gateway_stage" "strikes" {
  deployment_id = aws_api_gateway_deployment.strikes.id
  rest_api_id   = aws_api_gateway_rest_api.strikes.id

  stage_name = "v1"
}

# -----------------------------------------------------------------------------
# API-KEYS 
# -----------------------------------------------------------------------------
resource "aws_api_gateway_api_key" "strikes" {
  name = "strikes-api-key"
}

resource "aws_api_gateway_api_key" "dev" {
  name = "dev"
}

resource "aws_api_gateway_usage_plan" "strikes" {
  name         = "strikes-usage-plan"
  product_code = "MYCODE"

  api_stages {
    api_id = aws_api_gateway_rest_api.strikes.id
    stage  = aws_api_gateway_stage.strikes.stage_name
  }

  quota_settings {
    limit  = 20
    period = "DAY"
  }

  throttle_settings {
    burst_limit = 5
    rate_limit  = 10
  }
}

resource "aws_api_gateway_usage_plan_key" "main" {
  key_id        = aws_api_gateway_api_key.strikes.id
  key_type      = "API_KEY"
  usage_plan_id = aws_api_gateway_usage_plan.strikes.id
}

resource "aws_api_gateway_usage_plan" "dev" {
  name         = "dev"
  product_code = "MYCODE"

  api_stages {
    api_id = aws_api_gateway_rest_api.strikes.id
    stage  = aws_api_gateway_stage.strikes.stage_name
  }
}

resource "aws_api_gateway_usage_plan_key" "dev" {
  key_id        = aws_api_gateway_api_key.dev.id
  key_type      = "API_KEY"
  usage_plan_id = aws_api_gateway_usage_plan.dev.id
}

# -----------------------------------------------------------------------------
# WEBSOCKET API
# -----------------------------------------------------------------------------
resource "aws_apigatewayv2_api" "ws_strikes" {
  name                       = "ws-strikes"
  protocol_type              = "WEBSOCKET"
  route_selection_expression = "$request.body.action"
}

# -----------------------------------------------------------------------------
# CONNECT
# -----------------------------------------------------------------------------
resource "aws_apigatewayv2_route" "connect" {
  api_id    = aws_apigatewayv2_api.ws_strikes.id
  route_key = "$connect"
  target    = "integrations/${aws_apigatewayv2_integration.connect.id}"
}

resource "aws_apigatewayv2_integration" "connect" {
  api_id           = aws_apigatewayv2_api.ws_strikes.id
  integration_type = "AWS_PROXY"
  integration_uri  = module.lambdas.connect_lambda_invoke_arn
}

resource "aws_lambda_permission" "apigw_ws_invoke_connect_lambda" {
  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = module.lambdas.connect_lambda_function_name
  principal     = "apigateway.amazonaws.com"

  source_arn = "${aws_apigatewayv2_api.ws_strikes.execution_arn}/*/$connect"
}

# -----------------------------------------------------------------------------
# DISCONNECT
# -----------------------------------------------------------------------------
resource "aws_apigatewayv2_route" "disconnect" {
  api_id    = aws_apigatewayv2_api.ws_strikes.id
  route_key = "$disconnect"
  target    = "integrations/${aws_apigatewayv2_integration.disconnect.id}"
}

resource "aws_apigatewayv2_integration" "disconnect" {
  api_id           = aws_apigatewayv2_api.ws_strikes.id
  integration_type = "AWS_PROXY"
  integration_uri  = module.lambdas.disconnect_lambda_invoke_arn
}

resource "aws_lambda_permission" "apigw_ws_invoke_disconnect_lambda" {
  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = module.lambdas.disconnect_lambda_function_name
  principal     = "apigateway.amazonaws.com"

  source_arn = "${aws_apigatewayv2_api.ws_strikes.execution_arn}/*/$disconnect"
}

resource "aws_apigatewayv2_stage" "ws_strikes" {
  api_id = aws_apigatewayv2_api.ws_strikes.id
  name   = "v1"
}

resource "aws_apigatewayv2_deployment" "ws_strikes" {
  api_id = aws_apigatewayv2_api.ws_strikes.id

  lifecycle {
    create_before_destroy = true
  }
}

resource "aws_lambda_event_source_mapping" "example" {
  event_source_arn  = module.lambdas.strikes_db_stream_arn
  function_name     = module.lambdas.send_strikes_update_lambda_arn
  starting_position = "LATEST"
}
