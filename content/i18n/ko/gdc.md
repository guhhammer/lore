# gdc

GCC를 백엔드로 사용하는 D 컴파일러.

## Usage
- `gdc 경로/대상/소스.d -o 경로/대상/출력_실행파일` : 실행 파일을 생성
- `gdc -fdeps` : 모듈 종속성에 대한 정보를 출력
- `gdc -fdoc` : ddoc 문서를 생성
- `gdc -fintfc` : D 인터페이스 파일 생성
- `gdc -nostdlib` : 컴파일 시 표준 GCC 라이브러리를 링크하지 않음

## Links
- More information: https://wiki.dlang.org/Using_GDC
