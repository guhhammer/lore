# jj interdiff

두 리비전 간 변경 사항을 비교.

## Usage
- `jj interdiff -f revset` : 지정한 리비전과 작업 복사본의 변경 사항 비교
- `jj interdiff -f 시작_revset -t 끝_revset` : 지정한 두 리비전의 변경 사항 비교
- `jj interdiff -f 시작_revset -t 끝_revset filesets` : 지정한 경로의 변경 사항만 비교
- `jj interdiff -f revset -s` : 변경 사항 요약 표시
- `jj interdiff -f revset --stat` : 변경 사항 통계 표시
- `jj interdiff -f revset --git` : git 형식의 diff 표시
- `jj interdiff -f revset --color-words` : 색상만으로 변경 사항을 표시하는 단어 단위 diff 표시

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-interdiff
