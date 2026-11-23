# tee

`stdin`에서 읽고 `stdout` 및 파일(또는 명령어)로 쓰기.

## Usage
- `echo "example" | tee 경로/대상/파일` : `stdin`을 각 파일과 `stdout`으로 복사
- `echo "example" | tee -a 경로/대상/파일` : 주어진 파일에 덧붙이기, 덮어쓰지 않음
- `echo "example" | tee /dev/tty | xargs printf "[%s]"` : `stdin`을 터미널에 출력하고, 다른 프로그램으로 파이프하여 추가 처리
- `echo "example" | tee >(xargs mkdir) >(wc -c)` : "example"이라는 디렉터리 만들기, "example"의 문자 수 세기, "example"을 터미널에 쓰기

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/tee-invocation.html
