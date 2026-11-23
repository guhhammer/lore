# az logicapp

Azure Cloud Services에서 논리 앱을 관리. `azure-cli` (`az`라고도 함)의 일부.

## Usage
- `az logicapp create -n 이름 -g 리소스_그룹 -s 스토리지_계정` : 논리 앱 만들기
- `az logicapp delete -n 이름 -g 리소스_그룹` : 논리 앱 삭제
- `az logicapp list -g 리소스_그룹` : 논리 앱 나열
- `az logicapp restart -n 이름 -g 리소스_그룹` : 논리 앱을 다시 시작
- `az logicapp start -n 이름 -g 리소스_그룹` : 논리 앱을 시작
- `az logicapp stop -n 이름 -g 리소스_그룹` : 논리 앱을 중지

## Links
- More information: https://learn.microsoft.com/cli/azure/logicapp
