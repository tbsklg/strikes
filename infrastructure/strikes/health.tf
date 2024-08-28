locals {
  health_lambda_name = "health"
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

output "health_lambda_invoke_arn" {
  value = aws_lambda_function.health.invoke_arn
}

output "health_lambda_function_name" {
  value = aws_lambda_function.health.function_name
}

