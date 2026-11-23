# tac

파일을 역순으로 표시하고 연결.

## Usage
- `tac 경로/대상/파일1 경로/대상/파일2 ...` : 특정 파일들을 역순으로 연결
- `cat 경로/대상/파일 | tac` : `stdin`을 역순으로 표시
- `tac -s 구분자 경로/대상/파일1 경로/대상/파일2 ...` : 특정 separator 사용
- `tac -r -s 구분자 경로/대상/파일1 경로/대상/파일2 ...` : 특정 regex를 separator로 사용
- `tac -b 경로/대상/파일1 경로/대상/파일2 ...` : 각 파일 앞에 구분자 before 사용

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/tac-invocation.html
