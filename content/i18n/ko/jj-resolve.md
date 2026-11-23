# jj resolve

외부 병합 도구를 사용하여 충돌이 발생한 파일을 해결.

## Usage
- `jj resolve` : 모든 충돌 해결
- `jj resolve -l` : 모든 충돌 목록 표시
- `jj resolve -r revset` : 지정한 리비전의 충돌 해결
- `jj resolve 파일1 파일2 ...` : 지정한 파일의 충돌 해결
- `jj resolve --tool :theirs` : 들어오는 버전을 선택하여 충돌 해결
- `jj resolve --tool :ours` : 현재 버전을 선택하여 충돌 해결

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-resolve
