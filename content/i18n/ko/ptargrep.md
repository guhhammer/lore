# ptargrep

`.tar` 아카이브 파일에서 정규 표현식 패턴 찾기.

## Usage
- `ptargrep "검색_패턴" 경로/대상/파일1 경로/대상/파일2 ...` : 하나 이상의 `.tar` 아카이브 내에서 패턴 검색
- `ptargrep -b "검색_패턴" 경로/대상/파일` : 파일의 기본 이름을 사용하여 현재 디렉토리에 추출
- `ptargrep -i "검색_패턴" 경로/대상/파일` : `.tar` 아카이브 내에서 대소문자를 구분하지 않고 패턴 검색

## Links
- More information: https://manned.org/ptargrep
