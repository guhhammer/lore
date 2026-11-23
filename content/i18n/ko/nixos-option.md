# nixos-option

NixOS 설정을 확인합니다.

## Usage
- `nixos-option 옵션_키` : 주어진 옵션 키의 모든 하위 키 나열
- `nixos-option boot.kernelModules` : 현재 부팅 커널 모듈 나열
- `nixos-option users.users.사용자명.openssh.authorizedKeys.키_파일|키` : 특정 사용자의 인증된 키 나열
- `nixos-option nix.buildMachines` : 모든 원격 빌더 나열
- `NIXOS_CONFIG=경로/대상/configuration.nix nixos-option 옵션_키` : 다른 NixOS 설정에서 주어진 키의 모든 하위 키 나열
- `nixos-option -r users.users.사용자` : 사용자의 모든 값을 재귀적으로 표시

## Links
- More information: https://nixos.org/manual/nixos/stable/index.html#sec-modularity
