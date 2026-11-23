# jless

JSON 데이터를 대화형으로 탐색하고 확인.

## Usage
- `jless 경로/대상/파일.json` : JSON 파일 열기
- `cat 경로/대상/파일.json | jless` : `stdin`에서 JSON 데이터 읽기
- `jless --yaml 경로/대상/파일` : 파일 확장자와 관계없이 YAML 형식으로 파싱
- `jless -m line 경로/대상/파일.json` : 라인 모드로 시작 (중괄호, 쉼표 및 따옴표로 감싼 객체 키 표시)
- `jless -N 경로/대상/파일.json` : 줄 번호 숨기기
- `jless -r 경로/대상/파일.json` : 상대 줄 번호 표시
- `</>pattern` : [대화형] 패턴을 앞으로 검색 (`<n>`/`<N>`으로 다음/이전 일치 항목 이동)
- `<q>` : [대화형] 종료

## Links
- More information: https://jless.io/user-guide
