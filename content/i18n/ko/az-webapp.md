# az webapp

Azure Cloud Services에서 호스팅되는 웹 애플리케이션을 관리. `azure-cli`의 일부 (`az`라고도 함).

## Usage
- `az webapp list-runtimes -os windows|linux` : 웹 애플리케이션에 사용 가능한 런타임 나열
- `az webapp up -n 이름 -l 위치 -r 런타임` : 웹 애플리케이션 생성
- `az webapp list` : 모든 웹 애플리케이션 나열
- `az webapp delete -n 이름 -g 리소스_그룹` : 특정 웹 애플리케이션 삭제

## Links
- More information: https://learn.microsoft.com/cli/azure/webapp
