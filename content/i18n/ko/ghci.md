# ghci

Glasgow Haskell 컴파일러 대화형 환경.

## Usage
- `ghci` : REPL (대화형 쉘)을 시작
- `ghci 소스_파일.hs` : REPL을 시작하고 지정된 Haskell 소스 파일을 로드
- `ghci -X언어_옵션` : REPL을 시작하고 언어 옵션을 활성화
- `ghci -W경고_수준` : REPL을 시작하고 일젓 수준의 컴파일러 경고(예: `all` 또는 `compact`)를 활성화
- `ghci -i경로/대상/디렉터리1:경로/대상/디렉터리2:...` : 소스 파일을 찾기 위해 콜론으로 구분된 디렉터리 목록으로 REPL을 시작

## Links
- More information: https://downloads.haskell.org/ghc/latest/docs/users_guide/ghci.html
