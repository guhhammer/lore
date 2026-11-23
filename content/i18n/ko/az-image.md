# az image

Azure에서 사용자 지정 가상 머신 이미지를 관리. `azure-cli` (`az`라고도 함)의 일부.

## Usage
- `az image list -g 리소스_그룹` : 리소스 그룹 아래에 사용자 정의 이미지를 나열
- `az image create -g 리소스_그룹 -n 이름 --os-type windows|linux --source os_디스크_소스` : 관리 디스크 또는 스냅샷에서 사용자 지정 이미지를 생성
- `az image delete -n 이름 -g 리소스_그룹` : 사용자 정의 이미지 삭제
- `az image show -n 이름 -g 리소스_그룹` : 사용자 정의 이미지의 세부정보 표시
- `az image update -n 이름 -g 리소스_그룹 --set property=value` : 사용자 정의 이미지 업데이트

## Links
- More information: https://learn.microsoft.com/cli/azure/image
