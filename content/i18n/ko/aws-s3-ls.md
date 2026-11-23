# aws s3 ls

AWS S3 버킷, 폴더 (접두사) 및 파일 (객체) 나열.

## Usage
- `aws s3 ls` : 모든 버킷 나열
- `aws s3 ls s3://버킷_이름` : 버킷 루트 파일 및 폴더 나열 (`s3://`는 선택 사항)
- `aws s3 ls 버킷_이름/경로/대상/디렉터리//` : 디렉터리 내부에 있는 파일과 폴더를 직접 나열
- `aws s3 ls --recursive 버킷_이름` : 버킷의 모든 파일 나열
- `aws s3 ls --recursive 버킷_이름/경로/대상/디렉터리/접두사` : 주어진 접두사가 있는 경로의 모든 파일 나열
- `aws s3 ls help` : 도움말 표시

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/s3/ls.html
