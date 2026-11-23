# jj split

하나의 리비전을 두 개로 분할.

## Usage
- `jj split -r revision` : 지정한 리비전을 대화형으로 두 개로 분할하고, 두 번째 리비전을 첫 번째 리비전 위에 생성
- `jj split -r revision fileset` : 지정한 리비전에서 일치하는 파일만 분리하여 새 리비전 생성
- `jj split -r revision -d revset` : 지정한 리비전을 분할하고, 두 번째 리비전을 지정한 대상 리비전 위에 생성
- `jj split -r revision -B revset -A revset` : 지정한 리비전을 분할하고, 두 번째 리비전을 다른 리비전의 앞이나 뒤에 생성
- `jj split -r revision -p` : 지정한 리비전을 두 개의 병렬 리비전으로 분할

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-split
