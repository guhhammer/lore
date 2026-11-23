# terraform console

Terraform 표현식을 평가하기 위한 대화형 콘솔을 시작.

## Usage
- `terraform console` : 표현식을 평가하는 대화형 콘솔 시작
- `terraform console -plan` : 현재 상태 대신 계획된 상태를 기준으로 표현식 평가
- `echo "표현식" | terraform console` : 지정한 표현식을 비대화형으로 평가
- `terraform console -var '이름1=값1' -var '이름2=값2'` : 입력 변수 값 지정
- `terraform console -var-file 경로/대상/파일.tfvars` : 파일에서 입력 변수 값 불러오기

## Links
- More information: https://developer.hashicorp.com/terraform/cli/commands/console
