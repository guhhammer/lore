# jj rebase

리비전을 다른 부모 리비전으로 이동.

## Usage
- `jj rebase -r revset -o revset` : 지정한 리비전을 다른 부모 리비전으로 이동
- `jj rebase -s revset -o revset` : 지정한 리비전과 모든 자식 리비전을 다른 부모 리비전으로 이동
- `jj rebase -b revset -o revset` : 지정한 리비전이 포함된 브랜치의 모든 리비전을 다른 부모 리비전으로 이동
- `jj rebase -r revset -B revset -A revset` : 리비전을 다른 리비전의 앞이나 뒤로 이동

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-rebase
