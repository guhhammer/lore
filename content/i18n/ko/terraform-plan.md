# terraform plan

Terraform 실행 계획을 생성하고 보여줍니다.

## Usage
- `terraform plan` : 현재 디렉토리에서 실행 계획 생성 및 보기
- `terraform plan -destroy` : 현재 존재하는 모든 원격 객체를 삭제하는 계획 보기
- `terraform plan -refresh-only` : terraform 상태 및 출력 값을 업데이트하는 계획 보기
- `terraform plan -var '이름1=값1' -var '이름2=값2'` : 입력 변수에 대한 값 지정
- `terraform plan -target resource_type.resource_name[인스턴스 인덱스]` : 특정 리소스 하위 집합에만 Terraform의 주의 집중
- `terraform plan -json` : 계획을 JSON으로 출력
- `terraform plan -no-color >경로/대상/파일` : 특정 파일에 계획 기록

## Links
- More information: https://developer.hashicorp.com/terraform/cli/commands/plan
