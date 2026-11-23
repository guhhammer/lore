# objdump

오브젝트 파일에 대한 정보를 표시.

## Usage
- `objdump -f 경로/대상/바이너리` : 파일 헤더 정보를 표시
- `objdump -x 경로/대상/바이너리` : 모든 헤더 정보를 표시
- `objdump -d 경로/대상/바이너리` : 실행 가능한 섹션의 디스어셈블리 출력 표시
- `objdump -M intel -d 경로/대상/바이너리` : 인텔 구문으로 실행 가능한 섹션의 디스어셈블리 출력 표시
- `objdump -s 경로/대상/바이너리` : 모든 섹션의 전체 바이너리 헥스 덤프 표시

## Links
- More information: https://manned.org/objdump
