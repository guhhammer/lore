# cradle deploy

Cradle 배포 관리.

## Usage
- `cradle deploy production` : 서버에 Cradle을 배포
- `cradle deploy s3` : 아마존 S3에 정적 자산 배포
- `cradle deploy s3 --include-yarn` : yarn "components" 디렉토리를 포함하여 정적 자산 배포
- `cradle deploy s3 --include-upload` : "upload" 디렉토리를 포함한 정적 자산 배포
