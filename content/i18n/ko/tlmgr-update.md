# tlmgr update

TeX Live 패키지 업데이트.

## Usage
- `sudo tlmgr update --all` : 모든 TeX Live 패키지를 업데이트
- `sudo tlmgr update --self` : tlmgr 자체 업데이트
- `sudo tlmgr update 패키지` : 특정 패키지 업데이트
- `sudo tlmgr update --all --exclude 패키지` : 특정 패키지를 제외하고 모든 패키지 업데이트
- `sudo tlmgr update --all --backup` : 현재 패키지의 백업을 만들며 모든 패키지 업데이트
- `sudo tlmgr update --no-depends 패키지` : 의존성을 업데이트하지 않고 특정 패키지 업데이트
- `sudo tlmgr update --all --dry-run` : 아무런 변경 없이 모든 패키지 업데이트를 시뮬레이션

## Links
- More information: https://www.tug.org/texlive/doc/tlmgr.html#update-option...-pkg
