# nim

Nim 컴파일러. Nim 언어 소스 파일을 처리, 컴파일 및 링크합니다.

## Usage
- `nim c 경로/대상/파일.nim` : 소스 파일 컴파일
- `nim c -r 경로/대상/파일.nim` : 소스 파일 컴파일 및 실행
- `nim c -d:release 경로/대상/파일.nim` : 릴리스 최적화가 활성화된 상태로 소스 파일 컴파일
- `nim c -d:release --opt:size 경로/대상/파일.nim` : 파일 크기 최적화가 적용된 릴리스 바이너리 빌드
- `nim doc 경로/대상/파일.nim` : 모듈에 대한 HTML 문서 생성 (출력은 현재 디렉토리에 저장됨)
- `nim check 경로/대상/파일.nim` : 파일의 문법 및 의미 체계 검사

## Links
- More information: https://nim-lang.org/docs/nimc.html
