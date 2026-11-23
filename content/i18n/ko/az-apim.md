# az apim

Azure API Management 서비스를 관리. `azure-cli`의 일부 (`az`라고도 함).

## Usage
- `az apim list -g 리소스_그룹` : 리소스 그룹 내 API Management 서비스를 나열
- `az apim create -n 이름 -g 리소스_그룹 --publisher-email 이메일 --publisher-name 이름` : API Management 서비스 인스턴스 생성
- `az apim delete -n 이름 -g 리소스_그룹` : API Management 서비스 삭제
- `az apim show -n 이름 -g 리소스_그룹` : API Management 서비스 인스턴스의 세부정보 표시
- `az apim update -n 이름 -g 리소스_그룹` : API Management 서비스 인스턴스 업데이트

## Links
- More information: https://learn.microsoft.com/cli/azure/apim
