# ptghci

구문 강조, 여러 줄 편집, 실시간 타입 표시를 지원하는 대화형 Haskell REPL.

## Usage
- `ptghci` : 대화형 세션 시작
- `ptghci 경로/대상/파일.hs` : haskell 파일을 로드해 대화형 세션 시작
- `%past` : [대화형] 현재 세션의 이전 명령 목록 표시
- `%past -n 숫자` : [대화형] 이전 세션을 포함한 명령 기록 표시
- `%rerun 3,4-5,p8` : [대화형] 기록에서 지정한 명령 다시 실행
- `%hoogle 식별자` : [대화형] 식별자를 Hoogle에서 검색
- `%style 스타일_이름` : [대화형] 구문 강조 스타일 변경

## Links
- More information: https://github.com/litxio/ptghci
