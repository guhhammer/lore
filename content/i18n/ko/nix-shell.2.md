# nix-shell.2

Nix 표현을 기반으로 대화형 셸 시작.

## Usage
- `nix-shell` : 현재 디렉터리의 `shell.nix` 또는 `default.nix`의 nix 표현으로 시작
- `nix-shell --run "명령어 인수1 인수2 ..."` : 비대화형 셸에서 셸 명령 실행 후 종료
- `nix-shell default.nix` : 현재 디렉터리의 `default.nix`의 표현으로 시작
- `nix-shell -p 패키지1 패키지2 ...` : nixpkgs에서 로드된 패키지로 시작
- `nix-shell -p 패키지1 패키지2 ... -I nixpkgs=https://github.com/NixOS/nixpkgs/archive/nixpkgs_revision.tar.gz` : 특정 nixpkgs 리비전에서 로드된 패키지로 시작
- `nix-shell -i 인터프리터 -p 패키지1 패키지2 ...` : 특정 인터프리터에서 파일의 나머지를 평가하여 `#!-scripts`에서 사용 (자세한 내용은 <https://nixos.org/manual/nix/stable/#use-as-a-interpreter> 참고)

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/nix-shell.html
