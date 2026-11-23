# addr2line

바이너리의 주소를 파일 이름과 줄 번호로 변환.

## Usage
- `addr2line -e 경로/대상/실행_파일 주소` : 실행 파일의 명령어 주소로부터 소스 코드의 파일 이름과 줄 번호 표시
- `addr2line -e 경로/대상/실행_파일 -f 주소` : 함수 이름, 파일 이름 및 줄 번호 표시
- `addr2line -e 경로/대상/실행_파일 -f -C 주소` : C++ 코드의 함수 이름 디맹글링

## Links
- More information: https://manned.org/addr2line
