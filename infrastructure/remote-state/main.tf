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
}

resource "aws_s3_bucket" "tf_backend_state" {
  bucket = "tf-backend-state-strikes"
}

resource "aws_s3_bucket_versioning" "tf_backend_state_versioning" {
  bucket = aws_s3_bucket.tf_backend_state.id
  versioning_configuration {
    status = "Enabled"
  }
}

resource "aws_dynamodb_table" "tf_backend_lock" {
  name = "tf-backend-lock-strikes"
  hash_key = "LockID"
  read_capacity = 5
  write_capacity = 5
  attribute {
    name = "LockID"
    type = "S"
  }
}
