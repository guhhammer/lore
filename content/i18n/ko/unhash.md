# unhash

해시 테이블에 저장된 실행 파일 위치 등 항목 제거.

## Usage
- `unhash 명령어` : 해시 테이블에서 지정한 명령 제거
- `unhash -a 별칭` : 일반 별칭(aliases)의 해시 항목 제거
- `unhash -s 별칭` : 접미사 별칭(suffix aliases)의 해시 항목 제거
- `unhash -f 함수` : 셸 함수(functions)의 해시 항목 제거
- `unhash -d 디렉터리` : 디렉터리(directories)의 해시 항목 제거
- `unhash -f -m "패턴"` : 패턴과 일치하는(matches) 모든 함수의 해시 항목 제거

## Links
- More information: https://zsh.sourceforge.io/Doc/Release/Shell-Builtin-Commands.html
