locals {
  put_strike_lambda_name = "put-strikes"
  get_strikes_lambda_name = "get-strikes"
  delete_strikes_lambda_name = "delete-strikes"
  sse_strikes_lambda_name = "sse-strikes"
}

resource "aws_dynamodb_table" "strikes-table" {
  name           = "Strikes"
  billing_mode   = "PROVISIONED"
  read_capacity  = 8
  write_capacity = 8
  hash_key       = "UserId"

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
# PUT STRIKE
# -----------------------------------------------------------------------------
resource "aws_iam_role" "put_strike_lambda_role" {
  name               = "${local.put_strike_lambda_name}-role"
  assume_role_policy = data.aws_iam_policy_document.strikes_lambda_assume_role.json
  inline_policy {
    name = "dynamo_write"
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
  function_name = "${local.put_strike_lambda_name}"
  role          = aws_iam_role.put_strike_lambda_role.arn

  handler = "bootstrap"

  source_code_hash = data.archive_file.put_strike_lambda_archive.output_base64sha256

  runtime = "provided.al2023"

  architectures = ["x86_64"]

  memory_size = 1024
}

# -----------------------------------------------------------------------------
# GET STRIKES
# -----------------------------------------------------------------------------
resource "aws_iam_role" "get_strikes_lambda_role" {
  name               = "${local.get_strikes_lambda_name}-role"
  assume_role_policy = data.aws_iam_policy_document.strikes_lambda_assume_role.json
  inline_policy {
    name = "dynamo_write"
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
  function_name = "${local.get_strikes_lambda_name}"
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
resource "aws_iam_role" "delete_strikes_lambda_role" {
  name               = "${local.delete_strikes_lambda_name}-role"
  assume_role_policy = data.aws_iam_policy_document.strikes_lambda_assume_role.json
  inline_policy {
    name = "dynamo_delete"
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
  function_name = "${local.delete_strikes_lambda_name}"
  role          = aws_iam_role.delete_strikes_lambda_role.arn

  handler = "bootstrap"

  source_code_hash = data.archive_file.delete_strikes_lambda_archive.output_base64sha256

  runtime = "provided.al2023"

  architectures = ["x86_64"]

  memory_size = 1024
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
