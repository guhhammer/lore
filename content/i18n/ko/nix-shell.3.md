# nix-shell.3

지정된 패키지가 사용 가능한 셸 시작.

## Usage
- `nix shell nixpkgs#pkg1 nixpkgs#packageSet.pkg2 ...` : `nixpkgs`의 일부 패키지와 함께 대화형 셸 시작
- `nix shell nixpkgs/nixos-21.05#pkg` : `nixpkgs`의 이전 버전(21.05)에서 제공하는 패키지로 셸 시작
- `nix shell -L` : 현재 디렉터리의 플레이크에서 "기본 패키지"와 함께 셸 시작, 빌드가 발생하면 빌드 로그 출력
- `nix shell github:소유자/레포#pkg` : gitHub의 플레이크에서 패키지와 함께 셸 시작
- `nix shell nixpkgs#pkg -c 아무개_셸 --아무개_플래그 '다른 아무개 인수들'` : 패키지와 함께 셸에서 명령 실행

## Links
- More information: https://manned.org/nix3-shell
