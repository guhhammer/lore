# cargo build

로컬 패키지와 모든 종속성을 컴파일.

## Usage
- `cargo b` : 로컬 경로의 `Cargo.toml` 매니페이스트 파일에 의해 정의된 패키지를 빌드
- `cargo b -r` : 최적화를 통해 릴리스 모드에서 아티팩트 빌드
- `cargo b --locked` : `Cargo.lock`이 최신 버전이어야 함
- `cargo b --workspace` : 작업공간에서 모든 패키지를 빌드
- `cargo b -p 패키지` : 특정 패키지를 빌드
- `cargo b --bin 이름` : 지정된 바이너리만 빌드
- `cargo b --test 테스트이름` : 지정된 테스트 대상만 빌드

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-build.html
