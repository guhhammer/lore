# cargo run

현재 Cargo 패키지를 실행. 참고: 실행된 바이너리의 작업 디렉터리는 현재 작업 디렉터리로 설정됨.

## Usage
- `cargo r` : 기본 바이너리 타겟을 실행
- `cargo r --bin 이름` : 지정된 바이너리를 실행
- `cargo r --example 이름` : 지정된 예제를 실행
- `cargo r -F "기능1 기능2 ..."` : 공백 또는 쉼표로 구분된 기능 목록을 활성화
- `cargo r --no-default-features` : 기본 기능을 비활성화
- `cargo r --all-features` : 사용 가능한 모든 기능을 활성화
- `cargo r --profile 이름` : 주어진 프로필로 실행

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-run.html
