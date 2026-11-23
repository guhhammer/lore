# gh copilot

GitHub Copilot과 상호작용. 참고: 현재는 `copilot` 명령으로 대체되어 deprecated 상태.

## Usage
- `gh copilot suggest "git 설치"` : 설명을 기반으로 명령어 추천
- `gh copilot suggest -t git "Undo the most recent local commits"` : git 명령어 추천
- `gh copilot explain "traceroute example.com"` : 명령어 설명
- `echo 'eval "$(gh copilot alias -- bash)"' >>~/.bashrc` : bash용 쉘 별칭 생성
- `echo 'eval "$(gh copilot alias -- zsh)"' >>~/.zshrc` : zsh용 쉘 별칭 생성
- `gh copilot config` : 옵션 설정

## Links
- More information: https://github.com/github/gh-copilot#usage
