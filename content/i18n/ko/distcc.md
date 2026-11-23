# distcc

`distccd`와 함께 사용하는 분산 C/C++/ObjC 컴파일 클라이언트입니다.

## Usage
- `distcc 컴파일러명 -c 소스/파일/경로.c -o 출력/파일/경로.o` : `gcc`와 같은 컴파일러로 소스 파일을 컴파일
- `export DISTCC_HOSTS="localhost 호스트IP1 호스트IP2 ..."` : 컴파일을 분산시킬 원격 호스트 설정
- `make -j 병렬_작업_수 CC="distcc 컴파일러명"` : `distcc`를 사용해 `make`로 프로젝트를 병렬 컴파일
- `distcc --show-hosts` : 현재 설정된 `distcc` 호스트 목록 확인
- `distcc --help` : 도움말 표시
- `distcc --version` : 버전 표시

## Links
- More information: https://manned.org/distcc
