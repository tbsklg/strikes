locals {
  health_lambda_name              = "health"
  put_strike_lambda_name          = "put-strikes"
  get_strikes_lambda_name         = "get-strikes"
  delete_strikes_lambda_name      = "delete-strikes"
  sse_strikes_lambda_name         = "sse-strikes"
  website_lambda_name             = "website"
  connect_lambda_name             = "connect"
  disconnect_lambda_name          = "disconnect"
  send_strikes_update_lambda_name = "send_strikes_update"
}

module "website" {
  source = "../website"
}

# -----------------------------------------------------------------------------
#
# WEBSOCKET API
#
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
  integration_uri  = aws_lambda_function.connect.invoke_arn
}

resource "aws_lambda_permission" "apigw_ws_invoke_connect_lambda" {
  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.connect.function_name
  principal     = "apigateway.amazonaws.com"

  source_arn = "${aws_apigatewayv2_api.ws_strikes.execution_arn}/*/$connect"
}

resource "aws_iam_role" "connect_lambda_role" {
  name               = "${local.connect_lambda_name}-lambda-role"
  assume_role_policy = data.aws_iam_policy_document.strikes_lambda_assume_role.json
  inline_policy {
    name   = "connections_dynamo_write"
    policy = data.aws_iam_policy_document.connections_dynamo_write.json
  }
}

resource "aws_iam_role_policy_attachment" "connect_lambda_execution_role_policy_attachment" {
  role       = aws_iam_role.connect_lambda_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

data "archive_file" "connect_lambda_archive" {
  type        = "zip"
  source_file = "${path.module}/target/lambda/connect/bootstrap"
  output_path = "${path.module}/target/archive/connect.zip"
}

resource "aws_lambda_function" "connect" {
  filename      = data.archive_file.connect_lambda_archive.output_path
  function_name = local.connect_lambda_name
  role          = aws_iam_role.connect_lambda_role.arn

  handler = "bootstrap"

  source_code_hash = data.archive_file.connect_lambda_archive.output_base64sha256

  runtime = "provided.al2023"

  architectures = ["x86_64"]

  memory_size = 1024
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
  integration_uri  = aws_lambda_function.disconnect.invoke_arn
}

resource "aws_lambda_permission" "apigw_ws_invoke_disconnect_lambda" {
  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.disconnect.function_name
  principal     = "apigateway.amazonaws.com"

  source_arn = "${aws_apigatewayv2_api.ws_strikes.execution_arn}/*/$disconnect"
}

resource "aws_lambda_event_source_mapping" "example" {
  event_source_arn  = aws_dynamodb_table.strikes-table.stream_arn
  function_name     = aws_lambda_function.send_strikes_update.arn
  starting_position = "LATEST"
}

resource "aws_iam_role" "disconnect_lambda_role" {
  name               = "${local.disconnect_lambda_name}-lambda-role"
  assume_role_policy = data.aws_iam_policy_document.strikes_lambda_assume_role.json
  inline_policy {
    name   = "connections_dynamo_write"
    policy = data.aws_iam_policy_document.connections_dynamo_write.json
  }
}

resource "aws_iam_role_policy_attachment" "disconnect_lambda_execution_role_policy_attachment" {
  role       = aws_iam_role.disconnect_lambda_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

data "archive_file" "disconnect_lambda_archive" {
  type        = "zip"
  source_file = "${path.module}/target/lambda/disconnect/bootstrap"
  output_path = "${path.module}/target/archive/disconnect.zip"
}

resource "aws_lambda_function" "disconnect" {
  filename      = data.archive_file.disconnect_lambda_archive.output_path
  function_name = local.disconnect_lambda_name
  role          = aws_iam_role.disconnect_lambda_role.arn

  handler = "bootstrap"

  source_code_hash = data.archive_file.disconnect_lambda_archive.output_base64sha256

  runtime = "provided.al2023"

  architectures = ["x86_64"]

  memory_size = 1024
}

# -----------------------------------------------------------------------------
# CONNECTIONS DYNAMO DB
# -----------------------------------------------------------------------------
resource "aws_dynamodb_table" "connections-table" {
  name           = "Connections"
  billing_mode   = "PROVISIONED"
  read_capacity  = 8
  write_capacity = 8
  hash_key       = "ConnectionId"

  attribute {
    name = "ConnectionId"
    type = "S"
  }
}

data "aws_iam_policy_document" "connections_dynamo_write" {
  statement {
    effect = "Allow"

    actions = [
      "dynamodb:PutItem",
      "dynamodb:GetItem",
      "dynamodb:UpdateItem",
      "dynamodb:DeleteItem",
      "dynamodb:Scan"
    ]

    resources = [
      aws_dynamodb_table.connections-table.arn
    ]
  }
}

data "aws_iam_policy_document" "connections_dynamo_read_only" {
  statement {
    effect = "Allow"

    actions = [
      "dynamodb:GetItem",
      "dynamodb:Scan",
    ]

    resources = [
      aws_dynamodb_table.connections-table.arn
    ]
  }
}

# -----------------------------------------------------------------------------
# SEND STRIKES UPDATE
# -----------------------------------------------------------------------------
data "aws_iam_policy_document" "stream_access" {
  statement {
    actions = [
      "dynamodb:DescribeStream",
      "dynamodb:GetRecords",
      "dynamodb:GetShardIterator",
      "dynamodb:ListStreams",
    ]
    resources = ["*"]
  }
}

data "aws_iam_policy_document" "strikes_dynamo_read_only" {
  statement {
    effect = "Allow"

    actions = [
      "dynamodb:GetItem",
      "dynamodb:Scan",
    ]

    resources = [
      "*"
    ]
  }
}

data "aws_iam_policy_document" "manage_connections" {
  statement {
    actions   = ["execute-api:ManageConnections"]
    resources = ["*"]
  }
}

resource "aws_iam_role" "send_strikes_update_lambda_role" {
  name               = "${local.send_strikes_update_lambda_name}-lambda-role"
  assume_role_policy = data.aws_iam_policy_document.strikes_lambda_assume_role.json
  inline_policy {
    name   = "connections_dynamo_read"
    policy = data.aws_iam_policy_document.connections_dynamo_read_only.json
  }
  inline_policy {
    name   = "manage_connections"
    policy = data.aws_iam_policy_document.manage_connections.json
  }
  inline_policy {
    name   = "stream_access"
    policy = data.aws_iam_policy_document.stream_access.json
  }
  inline_policy {
    name   = "strikes_dynamo_read_access"
    policy = data.aws_iam_policy_document.strikes_dynamo_read_only.json
  }
}

resource "aws_iam_role_policy_attachment" "send_strikes_update_lambda_execution_role_policy_attachment" {
  role       = aws_iam_role.send_strikes_update_lambda_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

data "archive_file" "send_strikes_update_lambda_archive" {
  type        = "zip"
  source_file = "${path.module}/target/lambda/strikes_update/bootstrap"
  output_path = "${path.module}/target/archive/strikes_update.zip"
}

resource "aws_lambda_function" "send_strikes_update" {
  filename      = data.archive_file.send_strikes_update_lambda_archive.output_path
  function_name = local.send_strikes_update_lambda_name
  role          = aws_iam_role.send_strikes_update_lambda_role.arn

  handler = "bootstrap"

  source_code_hash = data.archive_file.send_strikes_update_lambda_archive.output_base64sha256

  runtime = "provided.al2023"

  architectures = ["x86_64"]

  memory_size = 1024

  environment {
    variables = {
      WEBSOCKET_API_ID = aws_apigatewayv2_api.ws_strikes.id
    }
  }
}

# -----------------------------------------------------------------------------
#
# REST API
#
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
  api_key_required = false
}

resource "aws_api_gateway_integration" "get_strikes" {
  http_method             = aws_api_gateway_method.get_strikes.http_method
  resource_id             = aws_api_gateway_resource.strikes.id
  rest_api_id             = aws_api_gateway_rest_api.strikes.id
  type                    = "AWS_PROXY"
  integration_http_method = "POST"
  uri                     = aws_lambda_function.get_strikes.invoke_arn
}

resource "aws_lambda_permission" "apigw_invoke_get_strikes_lambda" {
  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.get_strikes.function_name
  principal     = "apigateway.amazonaws.com"

  source_arn = "${aws_api_gateway_rest_api.strikes.execution_arn}/*/*"
}

resource "aws_iam_role" "get_strikes_lambda_role" {
  name               = "${local.get_strikes_lambda_name}-role"
  assume_role_policy = data.aws_iam_policy_document.strikes_lambda_assume_role.json
  inline_policy {
    name   = "dynamo_write"
    policy = data.aws_iam_policy_document.dynamo_read_only.json
  }
}

resource "aws_iam_role_policy_attachment" "get_strikes_basic_execution_role_policy_attachment" {
  role       = aws_iam_role.get_strikes_lambda_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

data "archive_file" "get_strikes_lambda_archive" {
  type        = "zip"
  source_file = "${path.module}/target/lambda/get_strikes/bootstrap"
  output_path = "${path.module}/target/archive/get_strikes.zip"
}

resource "aws_lambda_function" "get_strikes" {
  filename      = data.archive_file.get_strikes_lambda_archive.output_path
  function_name = local.get_strikes_lambda_name
  role          = aws_iam_role.get_strikes_lambda_role.arn

  handler = "bootstrap"

  source_code_hash = data.archive_file.get_strikes_lambda_archive.output_base64sha256

  runtime = "provided.al2023"

  architectures = ["x86_64"]

  memory_size = 1024
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
  uri                     = aws_lambda_function.delete_strikes.invoke_arn
}

resource "aws_lambda_permission" "apigw_invoke_delete_strikes_lambda" {
  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.delete_strikes.function_name
  principal     = "apigateway.amazonaws.com"

  source_arn = "${aws_api_gateway_rest_api.strikes.execution_arn}/*/*"
}

resource "aws_iam_role" "delete_strikes_lambda_role" {
  name               = "${local.delete_strikes_lambda_name}-role"
  assume_role_policy = data.aws_iam_policy_document.strikes_lambda_assume_role.json
  inline_policy {
    name   = "dynamo_delete"
    policy = data.aws_iam_policy_document.dynamo_delete.json
  }
}

resource "aws_iam_role_policy_attachment" "delete_strikes_basic_execution_role_policy_attachment" {
  role       = aws_iam_role.delete_strikes_lambda_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

data "archive_file" "delete_strikes_lambda_archive" {
  type        = "zip"
  source_file = "${path.module}/target/lambda/delete_strikes/bootstrap"
  output_path = "${path.module}/target/archive/delete_strikes.zip"
}

resource "aws_lambda_function" "delete_strikes" {
  filename      = data.archive_file.delete_strikes_lambda_archive.output_path
  function_name = local.delete_strikes_lambda_name
  role          = aws_iam_role.delete_strikes_lambda_role.arn

  handler = "bootstrap"

  source_code_hash = data.archive_file.delete_strikes_lambda_archive.output_base64sha256

  runtime = "provided.al2023"

  architectures = ["x86_64"]

  memory_size = 1024
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
  uri                     = aws_lambda_function.put_strike.invoke_arn
}

resource "aws_lambda_permission" "apigw_invoke_put_strike_lambda" {
  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.put_strike.function_name
  principal     = "apigateway.amazonaws.com"

  source_arn = "${aws_api_gateway_rest_api.strikes.execution_arn}/*/*"
}

resource "aws_iam_role" "put_strike_lambda_role" {
  name               = "${local.put_strike_lambda_name}-role"
  assume_role_policy = data.aws_iam_policy_document.strikes_lambda_assume_role.json
  inline_policy {
    name   = "dynamo_write"
    policy = data.aws_iam_policy_document.dynamo_write.json
  }
}

resource "aws_iam_role_policy_attachment" "strikes_basic_execution_role_policy_attachment" {
  role       = aws_iam_role.put_strike_lambda_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

data "archive_file" "put_strike_lambda_archive" {
  type        = "zip"
  source_file = "${path.module}/target/lambda/put_strike/bootstrap"
  output_path = "${path.module}/target/archive/put_strike.zip"
}

resource "aws_lambda_function" "put_strike" {
  filename      = data.archive_file.put_strike_lambda_archive.output_path
  function_name = local.put_strike_lambda_name
  role          = aws_iam_role.put_strike_lambda_role.arn

  handler = "bootstrap"

  source_code_hash = data.archive_file.put_strike_lambda_archive.output_base64sha256

  runtime = "provided.al2023"

  architectures = ["x86_64"]

  memory_size = 1024
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
  uri                     = aws_lambda_function.health.invoke_arn
}

resource "aws_lambda_permission" "apigw_invoke_health_lambda" {
  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.health.function_name
  principal     = "apigateway.amazonaws.com"

  source_arn = "${aws_api_gateway_rest_api.strikes.execution_arn}/*/*"
}

data "aws_iam_policy_document" "health_lambda_assume_role" {
  statement {
    effect = "Allow"

    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }

    actions = ["sts:AssumeRole"]
  }
}

resource "aws_iam_role" "health_lambda_role" {
  name               = "${local.health_lambda_name}-lambda-role"
  assume_role_policy = data.aws_iam_policy_document.health_lambda_assume_role.json
}

resource "aws_iam_role_policy_attachment" "health_lambda_basic_execution_role_policy_attachment" {
  role       = aws_iam_role.health_lambda_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

data "archive_file" "lambda_archive" {
  type        = "zip"
  source_file = "${path.module}/target/lambda/health/bootstrap"
  output_path = "${path.module}/target/archive/health.zip"
}

resource "aws_lambda_function" "health" {
  filename      = data.archive_file.lambda_archive.output_path
  function_name = local.health_lambda_name
  role          = aws_iam_role.health_lambda_role.arn

  handler = "bootstrap"

  source_code_hash = data.archive_file.lambda_archive.output_base64sha256

  runtime = "provided.al2023"

  architectures = ["x86_64"]

  memory_size = 1024
}

# -----------------------------------------------------------------------------
# WEBSITE
# -----------------------------------------------------------------------------
resource "aws_api_gateway_method" "website" {
  authorization    = "NONE"
  http_method      = "GET"
  resource_id      = aws_api_gateway_rest_api.strikes.root_resource_id
  rest_api_id      = aws_api_gateway_rest_api.strikes.id
  api_key_required = false
}

resource "aws_api_gateway_integration" "website" {
  http_method             = aws_api_gateway_method.website.http_method
  resource_id             = aws_api_gateway_rest_api.strikes.root_resource_id
  rest_api_id             = aws_api_gateway_rest_api.strikes.id
  type                    = "AWS_PROXY"
  integration_http_method = "POST"
  uri                     = aws_lambda_function.website.invoke_arn
}

resource "aws_lambda_permission" "apigw_invoke_website_lambda" {
  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.website.function_name
  principal     = "apigateway.amazonaws.com"

  source_arn = "${aws_api_gateway_rest_api.strikes.execution_arn}/*/*"
}

data "aws_iam_policy_document" "read_s3_website" {
  statement {
    effect    = "Allow"
    actions   = ["s3:GetObject"]
    resources = ["${module.website.s3_website_bucket_arn}/*"]
  }
}

resource "aws_iam_role" "website_lambda_role" {
  name               = "${local.website_lambda_name}-role"
  assume_role_policy = data.aws_iam_policy_document.strikes_lambda_assume_role.json
  inline_policy {
    name   = "read-website"
    policy = data.aws_iam_policy_document.read_s3_website.json
  }
}

resource "aws_iam_role_policy_attachment" "website_basic_execution_role_policy_attachment" {
  role       = aws_iam_role.website_lambda_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

data "archive_file" "website_lambda_archive" {
  type        = "zip"
  source_file = "${path.module}/target/lambda/website/bootstrap"
  output_path = "${path.module}/target/archive/website.zip"
}

resource "aws_lambda_function" "website" {
  filename      = data.archive_file.website_lambda_archive.output_path
  function_name = local.website_lambda_name
  role          = aws_iam_role.website_lambda_role.arn

  handler = "bootstrap"

  source_code_hash = data.archive_file.website_lambda_archive.output_base64sha256

  runtime = "provided.al2023"

  architectures = ["x86_64"]

  memory_size = 1024

  environment {
    variables = {
      REST_API_ID = aws_api_gateway_rest_api.strikes.id
      WEBSOCKET_API_ID = aws_apigatewayv2_api.ws_strikes.id
    }
  }
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
      aws_api_gateway_method.website.id,
      aws_api_gateway_integration.website.id,
    ]))
  }

  lifecycle {
    create_before_destroy = true
  }
}

resource "aws_apigatewayv2_deployment" "ws_strikes" {
  api_id = aws_apigatewayv2_api.ws_strikes.id

  triggers = {
    redeployment = sha1(jsonencode([
      aws_apigatewayv2_integration.connect.id,
      aws_apigatewayv2_integration.disconnect.id,
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

resource "aws_apigatewayv2_stage" "ws_strikes" {
  api_id = aws_apigatewayv2_api.ws_strikes.id
  name   = "v1"
}

# -----------------------------------------------------------------------------
# API-KEYS & USAGE PLANS
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
# STRIKES TABLE
# -----------------------------------------------------------------------------
resource "aws_dynamodb_table" "strikes-table" {
  name             = "Strikes"
  billing_mode     = "PROVISIONED"
  stream_enabled   = true
  stream_view_type = "KEYS_ONLY"
  read_capacity    = 8
  write_capacity   = 8
  hash_key         = "UserId"

  attribute {
    name = "UserId"
    type = "S"
  }
}

data "aws_iam_policy_document" "strikes_lambda_assume_role" {
  statement {
    effect = "Allow"

    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }

    actions = ["sts:AssumeRole"]
  }
}

data "aws_iam_policy_document" "dynamo_write" {
  statement {
    effect = "Allow"

    actions = [
      "dynamodb:PutItem",
      "dynamodb:GetItem",
      "dynamodb:UpdateItem",
      "dynamodb:Scan"
    ]

    resources = [
      aws_dynamodb_table.strikes-table.arn
    ]
  }
}

data "aws_iam_policy_document" "dynamo_read_only" {
  statement {
    effect = "Allow"

    actions = [
      "dynamodb:GetItem",
      "dynamodb:Scan",
    ]

    resources = [
      aws_dynamodb_table.strikes-table.arn
    ]
  }
}

data "aws_iam_policy_document" "dynamo_delete" {
  statement {
    effect = "Allow"

    actions = [
      "dynamodb:DeleteItem",
      "dynamodb:Scan",
    ]

    resources = [
      aws_dynamodb_table.strikes-table.arn
    ]
  }
}

# -----------------------------------------------------------------------------
# OUTPUTS
# -----------------------------------------------------------------------------
output "put_strike_lambda_invoke_arn" {
  value = aws_lambda_function.put_strike.invoke_arn
}

output "put_strike_lambda_function_name" {
  value = aws_lambda_function.put_strike.function_name
}

output "get_strikes_lambda_invoke_arn" {
  value = aws_lambda_function.get_strikes.invoke_arn
}

output "get_strikes_lambda_function_name" {
  value = aws_lambda_function.get_strikes.function_name
}

output "delete_strikes_lambda_invoke_arn" {
  value = aws_lambda_function.delete_strikes.invoke_arn
}

output "delete_strikes_lambda_function_name" {
  value = aws_lambda_function.delete_strikes.function_name
}

output "strikes_db_stream_arn" {
  value = aws_dynamodb_table.strikes-table.stream_arn
}

output "website_lambda_invoke_arn" {
  value = aws_lambda_function.website.invoke_arn
}

output "website_lambda_function_name" {
  value = aws_lambda_function.website.function_name
}
