# cargo test

Rust 패키지의 단위 및 통합 테스트를 실행.

## Usage
- `cargo t 테스트명` : 이름에 특정 문자열이 포함된 테스트만 실행
- `cargo t -- --test-threads 숫자` : 동시 실행 테스트 케이스 수 설정
- `cargo t -r` : 최적화를 통해, 릴리스 모드에서 아티팩트 테스트
- `cargo t --workspace` : 작업공간의 모든 패키지를 테스트
- `cargo t -p 패키지` : 특정 패키지에 대한 테스트를 실행
- `cargo t -- --nocapture` : 테스트 실행의 출력을 숨기지 않고 테스트를 실행

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-test.html
