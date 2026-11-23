# cargo clippy

일반적인 실수를 포착하고 Rust 코드를 개선하기 위한 린트 모음.

## Usage
- `cargo clippy` : 현재 디렉터리의 코드에 대한 검사를 실행
- `cargo clippy --locked` : `Cargo.lock`이 최신 버전이어야 함
- `cargo clippy --workspace` : 작업공간의 모든 패키지에 대해 검사 실행
- `cargo clippy --package 패키지` : 패키지에 대한 검사 실행
- `cargo clippy -- -W clippy::린트_그룹` : 린트 그룹에 대한 검사 실행 (<https://rust-lang.github.io/rust-clippy/stable/index.html#?groups=cargo,complexity,correctness,deprecated,nursery,pedantic,perf,restriction,style,suspicious> 참조)
- `cargo clippy -- -D warnings` : 경고를 오류로 처리
- `cargo clippy -- -A warnings` : 검사를 실행하고 경고를 무시
- `cargo clippy --fix` : clippy 제안을 자동으로 적용

## Links
- More information: https://github.com/rust-lang/rust-clippy
