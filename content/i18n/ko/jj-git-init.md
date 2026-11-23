# jj git init

Git 백엔드를 사용하는 새로운 Jujutsu 저장소를 생성.

## Usage
- `jj git init` : 현재 디렉터리에 Git 백엔드를 사용하는 새 저장소 생성
- `jj git init 경로/대상/디렉터리` : 지정한 디렉터리에 Git 백엔드를 사용하는 새 저장소 생성
- `jj git init --colocate` : jujutsu 저장소를 유효한 Git 저장소로 초기화 (`jj`와 `git` 명령어를 같은 디렉터리에서 모두 사용 가능)
- `jj git init --git-repo git_저장소` : 기존 Git 저장소를 백엔드로 사용하는 Jujutsu 저장소 초기화

## Links
- More information: https://docs.jj-vcs.dev/latest/cli-reference/#jj-git-init
