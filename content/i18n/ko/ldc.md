# ldc

LLVM을 백엔드로 사용하는 D 컴파일러.

## Usage
- `ldc2 경로/대상/소스.d -of=경로/대상/출력_실행파일` : 소스 코드 파일을 실행 가능한 바이너리로 컴파일
- `ldc2 -c 경로/대상/소스.d` : 소스 코드 파일을 링크하지 않고 컴파일
- `ldc -mtriple=아키텍처_os -c 경로/대상/소스.d` : 대상 아키텍처와 OS 선택
- `ldc2 -h` : 도움말 표시
- `ldc2 -help-hidden` : 전체 도움말 표시

## Links
- More information: https://wiki.dlang.org/Using_LDC
