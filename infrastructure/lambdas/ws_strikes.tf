locals {
  connect_lambda_name = "connect"
  disconnect_lambda_name = "disconnect"
  send_strikes_update_lambda_name = "send_strikes_update"
}

data "aws_iam_policy_document" "ws_lambda_assume_role" {
  statement {
    effect = "Allow"

    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }

    actions = ["sts:AssumeRole"]
  }
}

# -----------------------------------------------------------------------------
# CONNECTIONS DYANAMO DB
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
# CONNECT
# -----------------------------------------------------------------------------
resource "aws_iam_role" "connect_lambda_role" {
  name               = "${local.connect_lambda_name}-lambda-role"
  assume_role_policy = data.aws_iam_policy_document.ws_lambda_assume_role.json
  inline_policy {
    name = "connections_dynamo_write"
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
  function_name = "${local.connect_lambda_name}"
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
resource "aws_iam_role" "disconnect_lambda_role" {
  name               = "${local.disconnect_lambda_name}-lambda-role"
  assume_role_policy = data.aws_iam_policy_document.ws_lambda_assume_role.json
  inline_policy {
    name = "connections_dynamo_write"
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
  function_name = "${local.disconnect_lambda_name}"
  role          = aws_iam_role.disconnect_lambda_role.arn

  handler = "bootstrap"

  source_code_hash = data.archive_file.disconnect_lambda_archive.output_base64sha256

  runtime = "provided.al2023"

  architectures = ["x86_64"]

  memory_size = 1024
}

# -----------------------------------------------------------------------------
# SEND STRIKES UPDATE
# -----------------------------------------------------------------------------
data "aws_iam_policy_document" "manage_connections" {
  statement {
    actions   = ["execute-api:ManageConnections"]
    resources = ["*"]
  }
}

resource "aws_iam_role" "send_strikes_update_lambda_role" {
  name               = "${local.send_strikes_update_lambda_name}-lambda-role"
  assume_role_policy = data.aws_iam_policy_document.ws_lambda_assume_role.json
  inline_policy {
    name = "connections_dynamo_read"
    policy = data.aws_iam_policy_document.connections_dynamo_read_only.json
  }
  inline_policy {
    name = "manage_connections"
    policy = data.aws_iam_policy_document.manage_connections.json
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
  function_name = "${local.send_strikes_update_lambda_name}"
  role          = aws_iam_role.send_strikes_update_lambda_role.arn

  handler = "bootstrap"

  source_code_hash = data.archive_file.send_strikes_update_lambda_archive.output_base64sha256

  runtime = "provided.al2023"

  architectures = ["x86_64"]

  memory_size = 1024
}

# -----------------------------------------------------------------------------
# OUTPUTS
# -----------------------------------------------------------------------------
output "connect_lambda_invoke_arn" {
  value = aws_lambda_function.connect.invoke_arn
}

output "connect_lambda_function_name" {
  value = aws_lambda_function.connect.function_name
}

output "disconnect_lambda_invoke_arn" {
  value = aws_lambda_function.disconnect.invoke_arn
}

output "disconnect_lambda_function_name" {
  value = aws_lambda_function.disconnect.function_name
}
