# aws s3 website

Set the website configuration for a bucket.

## Usage
- `aws s3 website s3://bucket-name --index-document index.html` : configure a bucket as a static website
- `aws s3 website s3://bucket-name --index-document index.html --error-document error.html` : configure an error page for the website

## Categories
cloud, documents

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/s3/website.html
- tldr page: https://tldr.inbrowser.app/pages/common/aws-s3-website
