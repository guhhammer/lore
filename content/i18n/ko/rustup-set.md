# rustup set

`rustup` 설정 변경.

## Usage
- `rustup set default-host 호스트_트리플` : 기본 호스트 트리플 설정
- `rustup set profile minimal|default` : 기본 프로필 설정 (`minimal`은 `rustc`, `rust-std`, `cargo`만 포함하고, `default`는 `rust-docs`, `rustfmt`, `clippy`를 추가로 포함)
- `rustup set auto-self-update enable|disable|check-only` : `rustup update` 실행 시 `rustup` 자체 업데이트 여부 설정

## Links
- More information: https://rust-lang.github.io/rustup/
