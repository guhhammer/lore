# prowler aws

AWS의 보안 모범 사례 점검, 보안 감사, 규정 준수 검사 및 보고서 생성을 수행.

## Usage
- `prowler aws` : AWS 계정에 대해 기본 보안 검사 실행
- `prowler aws -p custom-profile -f us-east-1 eu-south-2 ...` : 사용지 지정 AWS 프로필을 사용하고 검사할 리전 필터링
- `prowler aws -s s3|ec2|...` : 지정한 AWS 서비스에 대해서만 보안 검사 실행
- `prowler aws -c s3_버킷_공개_접근` : 지정한 AWS 보안 검사 실행
- `prowler aws -e s3_버킷_공개_접근 --exclude-services s3|ec2|...` : 지정한 보안 검사 또는 서비스를 제외하고 실행

## Links
- More information: https://docs.prowler.com/user-guide/cli/tutorials/misc
