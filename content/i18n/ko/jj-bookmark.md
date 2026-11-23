# jj bookmark

`jj` 저장소의 북마크를 관리. Git 백엔드를 사용하는 경우, 북마크는 Git 브랜치에 해당.

## Usage
- `jj b c -r 리비전 이름` : 지정한 리비전에 새로운 북마크 생성
- `jj b l` : 모든 북마크와 대상 리비전 목록 표시
- `jj b m -t 리비전 이름` : 기존 북마크를 다른 리비전으로 이동
- `jj b t 이름@원격` : 지정한 원격 북마크 추적
- `jj b d 이름` : 북마크를 삭제하고, 다음 push 시 원격 저장소에도 삭제 사항 전파
- `jj b f 이름` : push 시 삭제를 전파하지 않고, 로컬에서만 북마크 제거

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-bookmark
