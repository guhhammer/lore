# az disk

Azure 관리 디스크를 관리. `azure-cli`의 일부(`az`라고도 함).

## Usage
- `az disk create -g 리소스_그룹 -n 디스크_이름 -z 기가바이트_크기` : 관리 디스크 만들기
- `az disk list -g 리소스_그룹` : 리소스 그룹의 관리 디스크 나열
- `az disk delete -g 리소스_그룹 -n 디스크_이름` : 관리 디스크 삭제
- `az disk grant-access -g 리소스_그룹 -n 디스크_이름 --access Read|Write --duration-in-seconds 초` : 관리 디스크에 대한 읽기 또는 쓰기 액세스 권한 부여 (내보내기 용)
- `az disk update -g 리소스_그룹 -n 디스크_이름 -z 새로운_기가바이트_크기` : 디스크 사이즈 업데이트

## Links
- More information: https://learn.microsoft.com/cli/azure/disk
