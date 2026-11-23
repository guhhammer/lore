# nix-env

Nix 사용자 환경을 조작하거나 조회합니다.

## Usage
- `nix-env -q` : 설치된 모든 패키지 나열
- `nix-env -q 검색어` : 설치된 패키지 조회
- `nix-env -qa 검색어` : 사용 가능한 패키지 조회
- `nix-env -iA nixpkgs.패키지_이름` : 패키지 설치
- `nix-env -i 패키지_이름 --file example.com` : URL에서 패키지 설치
- `nix-env -e 패키지_이름` : 패키지 제거
- `nix-env -u 패키지_이름` : 특정 패키지 업그레이드
- `nix-env -u` : 모든 패키지 업그레이드

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/nix-env.html
