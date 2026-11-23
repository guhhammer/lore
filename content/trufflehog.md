# trufflehog

Find and verify credentials in files, Git repositories, S3 buckets, and Docker images.

## Usage
- `trufflehog git https://github.com/trufflesecurity/test_keys --only-verified` : scan a Git repository for verified secrets
- `trufflehog github --org trufflesecurity --only-verified` : scan a GitHub organization for verified secrets
- `trufflehog git https://github.com/trufflesecurity/test_keys --only-verified --json` : scan a GitHub repository for verified keys and get JSON output
- `trufflehog github --repo https://github.com/trufflesecurity/test_keys --issue-comments --pr-comments` : scan a GitHub repository along with its Issues and Pull Requests
- `trufflehog s3 --bucket bucket_name --only-verified` : scan an S3 bucket for verified keys
- `trufflehog s3 --role-arn iam_role_arn` : scan S3 buckets using IAM Roles
- `trufflehog filesystem path/to/file_or_directory1 path/to/file_or_directory2 ...` : scan individual files or directories
- `trufflehog docker --image trufflesecurity/secrets --only-verified` : scan a Docker image for verified secrets

## Categories
containers, cloud, version-control

## Links
- More information: https://github.com/trufflesecurity/trufflehog#memo-usage
- tldr page: https://tldr.inbrowser.app/pages/common/trufflehog
