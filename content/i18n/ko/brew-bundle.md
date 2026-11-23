# brew bundle

Homebrew, Homebrew Cask 및 Mac App Store용 번들러.

## Usage
- `brew bundle` : 현재 경로의 Brewfile에서 패키지를 설치
- `brew bundle --file 경로/대상/파일` : 특정 경로의 특정 Brewfile에서 패키지를 설치
- `brew bundle dump` : 설치된 모든 패키지에서 Brewfile을 생성
- `brew bundle cleanup --force` : brewfile에 나열되지 않은 모든 공식을 제거
- `brew bundle check` : brewfile에 설치하거나 업그레이드할 항목이 있는지 확인
- `brew bundle list --all` : brewfile의 모든 항목을 나열

## Links
- More information: https://docs.brew.sh/Manpage#bundle-subcommand
