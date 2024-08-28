locals {
  put_strikes_lambda_name = "put-strikes"
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

data "aws_iam_policy_document" "put-strikes_lambda_assume_role" {
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
      "dynamodb:UpdateItem"
    ]

    resources = [
      aws_dynamodb_table.strikes-table.arn
    ]
  }
}

resource "aws_iam_role_policy_attachment" "strikes_basic_execution_role_policy_attachment" {
  role       = aws_iam_role.put_strikes_lambda_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

resource "aws_iam_role" "put_strikes_lambda_role" {
  name               = "${local.put_strikes_lambda_name}-role"
  assume_role_policy = data.aws_iam_policy_document.put-strikes_lambda_assume_role.json
  inline_policy {
    name   = "dynamo_write"
    policy = data.aws_iam_policy_document.dynamo_write.json
  }
}

data "archive_file" "put_strikes_lambda_archive" {
  type        = "zip"
  source_file = "${path.module}/target/lambda/put_strikes/bootstrap"
  output_path = "${path.module}/target/archive/put_strikes.zip"
}

resource "aws_lambda_function" "put_strikes" {
  filename      = data.archive_file.put_strikes_lambda_archive.output_path
  function_name = local.put_strikes_lambda_name
  role          = aws_iam_role.put_strikes_lambda_role.arn

  handler = "bootstrap"

  source_code_hash = data.archive_file.put_strikes_lambda_archive.output_base64sha256

  runtime = "provided.al2023"

  architectures = ["x86_64"]

  memory_size = 1024
}

output "put_strikes_lambda_invoke_arn" {
  value = aws_lambda_function.put_strikes.invoke_arn
}

output "put_strikes_lambda_function_name" {
  value = aws_lambda_function.put_strikes.function_name
}
