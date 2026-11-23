# aws s3 rm

S3 객체 삭제.

## Usage
- `aws s3 rm s3://버킷_이름/경로/대상/파일` : 특정 S3 객체 삭제
- `aws s3 rm s3://버킷_이름/경로/대상/파일 --dryrun` : 특정 S3 객체를 삭제하지 않고 삭제 결과를 미리보기 (dry-run)
- `aws s3 rm s3://arn:aws:s3:리전:계정_아이디:액세스_포인트/액세스_포인트_이름/객체_키` : 특정 S3 액세스 포인트에서 객체 삭제
- `aws s3 rm s3://버킷_이름 --recursive` : 버킷 내 모든 객체 삭제 (버킷 비우기)
- `aws s3 rm help` : 도움말 표시

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/s3/rm.html
