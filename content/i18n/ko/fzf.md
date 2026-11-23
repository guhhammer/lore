# fzf

명령줄 퍼지 찾기.

## Usage
- `find 경로/대상/디렉터리 -type f | fzf` : 지정된 디렉터리의 모든 파일에서 `fzf`를 시작
- `ps aux | fzf` : 프로세스 실행을 위해 `fzf`를 시작
- `find 경로/대상/디렉터리 -type f | fzf -m >경로/대상/파일` : `<Shift Tab>`을 사용해 여러 파일을 선택하고 파일에 작성
- `fzf -q "쿼리"` : 지정된 쿼리로 `fzf`를 시작
- `fzf -q "^core go$ | rb$ | py$"` : `core`로 시작하고 `go`, `rb` 또는 `py`로 끝나는 항목에서 `fzf`를 시작
- `fzf -q '!pyc travis'` : pvc와 일치하지 않고 travis와 정확히 일치하는 항목에 대해 `fzf`를 시작

## Links
- More information: https://github.com/junegunn/fzf#usage
