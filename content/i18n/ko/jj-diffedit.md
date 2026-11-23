# jj diffedit

diff 편집기를 사용하여 리비전의 변경 내용을 수정.

## Usage
- `jj diffedit` : diff 편집기로 현재 리비전의 변경 내용 수정
- `jj diffedit -r revset` : 지정한 리비전의 변경 내용 수정
- `jj diffedit -f 시작_revset -t 끝_revset` : 두 리비전 간의 변경 내용을 비교하며 수정
- `jj diffedit filesets` : 지정한 경로의 변경 내용만 수정 (일치하지 않는 경로는 변경되지 않음)
- `jj diffedit --tool 이름` : 지정한 diff 편집기 사용
- `jj diffedit --restore-descendants` : 자식 리비전을 리베이스할 때 diff 대신 내용 유지

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-diffedit
