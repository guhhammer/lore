# jj restore

다른 리비전의 파일을 복원.

## Usage
- `jj restore -f revset -t revset filesets` : 지정한 리비전의 파일을 다른 리비전으로 복원
- `jj restore -c revset filesets` : 부모 리비전들의 병합 결과를 기준으로 지정한 리비전의 변경 사항 되돌리기
- `jj restore -f revset -t revset -i` : 복원할 내용을 대화형으로 선택하여 복원

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-restore
