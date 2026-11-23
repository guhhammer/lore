# nix

패키지 관리를 신뢰성 있고, 재현 가능하며, 선언적으로 만드는 강력한 패키지 관리자. `build`, `develop`, `flake`, `registry`, `profile`, `search`, `repl`, `store`, `edit`, `why-depends` 등의 일부 하위 명령에는 자체 사용 설명서가 있습니다.

## Usage
- `mkdir -p ~/.config/nix; echo 'experimental-features = nix-command flakes' >~/.config/nix/nix.conf` : `nix` 명령 활성화
- `nix search nixpkgs 검색어` : nixpkgs에서 이름이나 설명으로 패키지 검색
- `nix shell nixpkgs#패키지1 nixpkgs#패키지2 nixpkgs#패키지3 ...` : nixpkgs에서 지정한 패키지가 사용 가능한 셸 시작
- `nix profile install nixpkgs#패키지1 nixpkgs#패키지2 nixpkgs#패키지3 ...` : nixpkgs에서 일부 패키지를 영구적으로 설치
- `nix store gc` : nix 저장소에서 사용하지 않는 경로 제거하여 공간 확보
- `nix repl` : nix 표현식을 평가하기 위한 대화형 환경 시작
- `nix help 하위_명령` : 특정 하위 명령에 대한 도움말 표시

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/new-cli/nix
