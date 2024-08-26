locals {
  app_name    = "strikes"
  put_strikes_lambda_name = "put_strikes"
}

data "aws_iam_policy_document" "lambda_assume_role" {
  statement {
    effect = "Allow"

    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }

    actions = ["sts:AssumeRole"]
  }
}

resource "aws_iam_role_policy_attachment" "basic_execution_role_policy_attachment" {
  role       = aws_iam_role.put_strikes_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

resource "aws_iam_role" "put_strikes_role" {
  name               = "${local.app_name}-${local.put_strikes_lambda_name}"
  assume_role_policy = data.aws_iam_policy_document.lambda_assume_role.json
}

data "archive_file" "put_strikes_lambda_archive" {
  type        = "zip"
  source_file = "${path.module}/target/lambda/put_strikes/bootstrap"
  output_path = "${path.module}/target/archive/health.zip"
}

resource "aws_lambda_function" "put_strikes" {
  filename      = data.archive_file.put_strikes_lambda_archive.output_path
  function_name = "${local.app_name}-${local.put_strikes_lambda_name}"
  role          = aws_iam_role.put_strikes_role.arn

  handler = "bootstrap"

  source_code_hash = data.archive_file.put_strikes_lambda_archive.output_base64sha256

  runtime = "provided.al2023"

  architectures = ["arm64"]

  memory_size = 1024
}

output "put_strikes_lambda_invoke_arn" {
  value = aws_lambda_function.put_strikes.invoke_arn
}

output "put_strikes_lambda_function_name" {
  value = aws_lambda_function.put_strikes.function_name
}
