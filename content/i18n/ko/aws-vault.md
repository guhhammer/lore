# aws-vault

개발 환경에서 AWS 자격 증명을 안전하게 저장하고 액세스하기 위한 저장소.

## Usage
- `aws-vault add 프로파일` : 보안 키 저장소에 자격 증명을 추가
- `aws-vault exec 프로파일 -- aws s3 ls` : 환경에서 AWS 자격 증명을 사용하여 명령을 실행
- `aws-vault login 프로파일` : 브라우저 창을 열고 AWS 콘솔에 로그인
- `aws-vault list` : 자격 증명 및 세션과 함께 프로필을 나열
- `aws-vault rotate 프로파일` : AWS 자격 증명 교체
- `aws-vault remove 프로파일` : 보안 키 저장소에서 자격 증명을 제거

## Links
- More information: https://github.com/99designs/aws-vault
