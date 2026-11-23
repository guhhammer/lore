# cat

파일 출력 및 연결.

## Usage
- `cat 경로/대상/파일` : 파일 내용을 `stdout`으로 출력
- `cat 경로/대상/파일1 경로/대상/파일2 ... >경로/대상/출력_파일` : 여러 파일을 출력 파일로 연결
- `cat 경로/대상/파일1 경로/대상/파일2 ... >>경로/대상/출력_파일` : 여러 파일을 출력 파일에 추가
- `cat -u /dev/tty12 >/dev/tty13` : 버퍼링 없이 파일 내용을 출력 파일로 복사
- `cat - >경로/대상/파일` : `stdin`을 파일로 쓰기

## Links
- More information: https://manned.org/cat.1posix
