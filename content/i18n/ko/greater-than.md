# greater-than

출력을 파일로 리디렉션.

## Usage
- `명령어 >경로/대상/파일` : `stdout`을 파일로 리디렉션
- `명령어 >>경로/대상/파일` : 파일에 추가
- `명령어 &>경로/대상/파일` : `stdout`과 `stderr`을 모두 파일로 리디렉션함
- `명령어 &>/dev/null` : `stdout` 및 `stderr`을 모두 `/dev/null`로 리디렉션하여, 터미널 출력을 깨끗하게 유지
- `>경로/대상/파일` : 파일 내용을 지우거나 새로운 빈 파일을 생성

## Links
- More information: https://gnu.org/software/bash/manual/bash.html#Redirecting-Output
