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

module "lambdas" {
  source = "./lambdas"
}
