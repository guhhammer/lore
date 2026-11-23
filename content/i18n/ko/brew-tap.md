# brew tap

Homebrew formula 저장소를 추가. 인자를 지정하지 않으면, 설치된 모든 tap 목록을 출력.

## Usage
- `brew tap` : 설치된 Homebrew tap 목록 출력
- `brew tap github_사용자명/github_저장소_이름` : gitHub에 호스팅된 Git 저장소 추가(tap)
- `brew tap 사용자명/저장소_이름 git_clone_url` : gitHub 외부에 호스팅된 Git 저장소 추가(tap)
- `brew tap 사용자명/저장소_이름 https://gitlab.com/사용자명/저장소_이름.git` : gitLab에 호스팅된 저장소 tap 추가
- `brew tap -h` : 도움말 표시

## Links
- More information: https://docs.brew.sh/Manpage#tap-options-userrepo-url
