# jj diff

두 리비전 사이의 파일 내용을 비교.

## Usage
- `jj diff` : 현재 리비전의 변경 사항 표시
- `jj diff -r revsets` : 지정한 revset의 변경 사항 표시 (예: `B::D`, `A..D`, `B|C|D` 등)
- `jj diff -f 시작_revset -t 끝_revset` : 지정한 리비전 사이의 변경 사항 표시
- `jj diff --stat` : 변경 사항 통계 표시
- `jj diff --git` : git 형식의 Diff 표시

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-diff
