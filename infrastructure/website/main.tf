resource "aws_s3_bucket" "website" {
  bucket        = "website-483220362587"
  force_destroy = true
}

resource "aws_s3_bucket_public_access_block" "public_access_block" {
  bucket = aws_s3_bucket.website.id

  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

resource "aws_s3_object" "index" {
  bucket       = aws_s3_bucket.website.id
  key          = "index.html.hbs"
  source       = "website/templates/index.html.hbs"
  content_type = "text/html"

  etag = filemd5("website/templates/index.html.hbs")
}

output "s3_website_bucket_arn" {
  value = aws_s3_bucket.website.arn
}
