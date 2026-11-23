# az group

리소스 그룹 및 템플릿 배포를 관리. `azure-cli`의 일부 (`az`라고도 함).

## Usage
- `az group create -n 이름 -l 위치` : 새로운 리소스 그룹 생성
- `az group exists -n 이름` : 리소스 그룹이 있는지 확인
- `az group delete -n 이름` : 리소스 그룹 삭제
- `az group wait -n 이름 --created|deleted|exists|updated` : 리소스 그룹의 조건이 충족될 때까지 기다림

## Links
- More information: https://learn.microsoft.com/cli/azure/group
