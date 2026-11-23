# nix edit

Nix 패키지의 Nix 표현을 `$EDITOR`에서 엽니다.

## Usage
- `nix edit nixpkgs#패키지` : nixpkgs에서 패키지의 Nix 표현 소스를 `$EDITOR`에서 열기
- `EDITOR=cat nix edit nixpkgs#패키지` : 패키지의 소스를 `stdout`으로 덤프

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/new-cli/nix3-edit.html
