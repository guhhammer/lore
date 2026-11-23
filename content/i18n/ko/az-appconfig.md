# az appconfig

Azure에서 앱 구성을 관리. `azure-cli`의 일부 (`az`라고도 함).

## Usage
- `az appconfig create -n 이름 -g 그룹_이름 -l 위치` : 앱 구성 만들기
- `az appconfig delete -g 리소스그룹_이름 -n 앱구성파일_이름` : 특정 앱 구성 삭제
- `az appconfig list` : 현재 구독 아래의 모든 앱 구성을 나열
- `az appconfig list -g 리소스그룹_이름` : 특정 리소스 그룹 아래 모든 앱 구성을 나열
- `az appconfig show -n 앱구성파일_이름` : 앱 구성의 속성 표시
- `az appconfig update -g 리소스그룹_이름 -n 앱구성파일_이름` : 특정 앱 구성 업데이트

## Links
- More information: https://learn.microsoft.com/cli/azure/appconfig
