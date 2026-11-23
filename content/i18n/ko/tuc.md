# tuc

구분자가 일치하는 위치를 기준으로 텍스트 (또는 바이트)를 나누고, 원하는 부분만 유지. 합리적인 기본값을 제공하는 `cut`보다 사용하기 쉽고 강력한 도구.

## Usage
- `echo "foo bar baz" | tuc -d ' ' -f 3,2,1` : 필드를 분리하고 순서를 재배치
- `echo "foo bar baz" | tuc -d ' ' -r ' ➡ '` : 구분자인 `space`를 화살표로 변경
- `echo "foo bar    baz" | tuc -d ' ' -f 2:` : 지정한 범위의 필드만 유지
- `echo "a,b, c" | tuc -e '[, ]+' -f 1,3` : `regex`을 사용하여 필드 분리
- `echo "foo bar baz" | tuc -d ' ' --json` : JSON 형식으로 출력

## Links
- More information: https://github.com/riquito/tuc#help
