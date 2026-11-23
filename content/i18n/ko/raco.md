# raco

Racket 명령줄 도구.

## Usage
- `raco pkg install --auto 패키지_출처` : 패키지를 설치하고, 자동으로 의존성을 설치
- `raco pkg install` : 현재 디렉토리를 패키지로 설치
- `raco setup 컬렉션1 컬렉션2 ...` : 컬렉션에 대한 바이트코드, 문서, 실행 파일 및 메타데이터 색인 생성(또는 재생성)
- `raco test 경로/대상/테스트1.rkt 경로/대상/테스트2.rkt ...` : 파일 내 테스트 실행
- `raco docs 검색어 ...` : 로컬 문서 검색
- `raco help` : 도움말 표시

## Links
- More information: https://docs.racket-lang.org/raco/
