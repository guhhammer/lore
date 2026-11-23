# hostess

`/etc/hosts` 파일 관리.

## Usage
- `hostess list` : 도메인, 대상 IP 주소 및 켜기/끄기 상태를 나열
- `hostess add local.example.com 127.0.0.1` : 호스트 파일에 컴퓨터를 가리키는 도메인을 추가
- `hostess del local.example.com` : 호스트 파일에서 도메인을 제거
- `hostess off local.example.com` : 도메인 비활성화(제거하지는 않음)

## Links
- More information: https://github.com/cbednarski/hostess
