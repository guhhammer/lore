# choco source

Chocolatey 패키지 소스 관리.

## Usage
- `choco source list` : 현재 사용 가능한 소스 나열
- `choco source add -n 이름 -s url` : 새 패키지 소스 추가
- `choco source add -n 이름 -s url -u 사용자_명 -p 비밀번호` : 자격 증명을 사용하여 새 패키지 소스 추가
- `choco source add -n 이름 -s url --cert 경로\대상\인증서_파일` : 클라이언트 인증서를 사용하여 새 패키지 소스 추가
- `choco source enable -n 이름` : 패키지 소스 활성화
- `choco source disable -n 이름` : 패키지 소스 비활성화
- `choco source remove -n 이름` : 패키지 소스 제거

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/source/
