terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.38.0"
    }
  }
  backend "s3" {
    bucket = "tf-backend-state-strikes"
    encrypt = true
    dynamodb_table = "tf-backend-lock-strikes"
    key = "terraform.tfstate"
    region = "eu-central-1"
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
  authorization     = "NONE"
  http_method       = "GET"
  resource_id       = aws_api_gateway_resource.health.id
  rest_api_id       = aws_api_gateway_rest_api.strikes.id
  api_key_required  = true
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

resource "aws_api_gateway_api_key" "strikes" {
  name = "strikes-api-key"
}

resource "aws_api_gateway_stage" "strikes" {
  deployment_id = aws_api_gateway_deployment.strikes.id
  rest_api_id   = aws_api_gateway_rest_api.strikes.id

  stage_name    = "v1"
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
