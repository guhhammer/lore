# bzgrep

`grep`을 사용하여 `bzip2`로 압축된 파일에서 패턴을 찾음.

## Usage
- `bzgrep "검색_패턴" 경로/대상/파일` : 압축 파일 내에서 패턴 검색
- `bzgrep -r "검색_패턴" 경로/대상/tar/파일` : bzip2로 압축된 `.tar` 아카이브에서 파일을 반복적으로 검색하여 패턴을 찾음
- `bzgrep --context|--before-context|--after-context 3 "검색_패턴" 경로/대상/파일` : 각 일치하는 전후 및 전후 3줄의 컨텍스트를 출력
- `bzgrep -H -n "검색_패턴" 경로/대상/파일` : 각 일치 항목의 파일 이름과 줄 번호를 출력
- `bzgrep -o "검색_패턴" 경로/대상/파일` : 패턴과 일치하는 줄을 검색하여 일치하는 텍스트만 출력
- `cat path/to/bz_compressed_file | bzgrep -v "검색_패턴"` : 패턴과 일치하지 않는 줄을 `stdin`으로 검색
- `bzgrep -E -i "검색_패턴" 경로/대상/파일` : 대소문자를 구분하지 않는 모드에서 확장 정규표현식 (`?`, `+`, `{}`, `()` 및 `|` 지원)을 사용

## Links
- More information: https://manned.org/bzgrep
