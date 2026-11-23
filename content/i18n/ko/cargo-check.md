# cargo check

로컬 패키지와 모든 종속 항목에 오류가 있는지 확인.

## Usage
- `cargo c` : 현재 패키지 검사
- `cargo c --tests` : 모든 테스트 검사
- `cargo c --test 통합_테스트1` : `tests/integration_test1.rs`에서 통합 테스트를 확인
- `cargo c -F 기능1,기능2` : `feature1` 및 `feature2` 기능이 포함된 현재 패키지를 확인
- `cargo c --no-default-features` : 기본 기능이 비활성화된 현재 패키지 확인

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-check.html
