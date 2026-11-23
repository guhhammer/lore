# jj arrange

커밋 그래프를 대화형으로 재배치.

## Usage
- `jj arrange` : `revsets.arrange`에 설정된 커밋을 대화형으로 재배치 (기본값은 변경 가능한 커밋)
- `jj arrange -r revset` : 지정한 리비전을 대화형으로 재배치
- `jj arrange -r revset1 -r revset2` : 여러 revset을 대화형으로 재배치
- `jj arrange -r 'trunk()..@'` : `trunk`부터 현재 리비전까지의 커밋 스택을 대화형으로 재배치

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-arrange
