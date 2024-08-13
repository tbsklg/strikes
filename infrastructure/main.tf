terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.38.0"
    }
  }
}

provider "aws" {
  region = "eu-central-1"
  profile = "cc"
}

module "health" {
  source = "./health"
}

resource "aws_api_gateway_rest_api" "strikes" {
  name = "strikes"
}

resource "aws_api_gateway_resource" "health" {
  path_part   = "health"
  parent_id   = aws_api_gateway_rest_api.strikes.root_resource_id
  rest_api_id = aws_api_gateway_rest_api.strikes.id
}

resource "aws_api_gateway_method" "health" {
  authorization = "NONE"
  http_method   = "GET"
  resource_id   = aws_api_gateway_resource.health.id
  rest_api_id   = aws_api_gateway_rest_api.strikes.id
}

resource "aws_api_gateway_integration" "health" {
  http_method = aws_api_gateway_method.health.http_method
  resource_id = aws_api_gateway_resource.health.id
  rest_api_id = aws_api_gateway_rest_api.strikes.id
  type        = "AWS_PROXY"
  integration_http_method = "POST"
  uri                     = "${module.health.lambda_invoke_arn}"
}

resource "aws_lambda_permission" "apigw_invoke_health_lambda" {
  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = "${module.health.lambda_function_name}"
  principal     = "apigateway.amazonaws.com"

  source_arn = "${aws_api_gateway_rest_api.strikes.execution_arn}/*/*"
}

resource "aws_api_gateway_deployment" "strikes" {
  rest_api_id = aws_api_gateway_rest_api.strikes.id

  triggers = {
    redeployment = sha1(jsonencode([
      aws_api_gateway_resource.health.id,
      aws_api_gateway_method.health.id,
      aws_api_gateway_integration.health.id,
    ]))
  }

  lifecycle {
    create_before_destroy = true
  }
}

resource "aws_api_gateway_stage" "strikes" {
  deployment_id = aws_api_gateway_deployment.strikes.id
  rest_api_id   = aws_api_gateway_rest_api.strikes.id
  stage_name    = "v1"
}
