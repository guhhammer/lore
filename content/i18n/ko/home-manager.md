# home-manager

Nix를 사용하여 사용자별 환경을 관리하며, 홈 디렉토리를 선언적으로 구성할 수 있게 해주는 도구.

## Usage
- `home-manager build` : `~/.config/nixpkgs/home.nix`에 정의된 설정을 적용하지 않고 빌드
- `home-manager switch` : 새로운 설정을 빌드하고 적용(전환)
- `home-manager rollback` : 이전 설정 버전으로 롤백
- `home-manager generations` : 모든 설정 버전(generation) 목록 출력
- `home-manager 명령어 --flake 경로/대상/flake` : flakes 사용 시, flake 경로를 지정하여 명령 실행 (build, switch, news 등)

## Links
- More information: https://github.com/nix-community/home-manager
