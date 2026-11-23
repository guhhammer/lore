# wc

줄 단어 및 바이트 수 계산.

## Usage
- `wc -l 경로/대상/파일` : 파일의 모든 줄 수 계산
- `wc -w 경로/대상/파일` : 파일의 모든 단어 수 계산
- `wc -c 경로/대상/파일` : 파일의 모든 바이트 수 계산
- `wc -m 경로/대상/파일` : 파일의 모든 문자 수 계산(멀티바이트 문자 고려)
- `find . | wc` : `stdin`의 모든 줄, 단어 및 바이트 수를 계산
- `wc -L 경로/대상/파일` : 가장 긴 줄의 길이를 문자 수로 계산

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/wc-invocation.html
