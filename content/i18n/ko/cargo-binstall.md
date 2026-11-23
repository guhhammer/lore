# cargo binstall

CI 아티팩트에서 Rust 바이너리를 설치. 사용 가능한 바이너리가 없을 경우 `cargo install` (소스 코드 빌드)로 대체됨.

## Usage
- `cargo binstall 패키지` : <https://crates.io>에서 패키지 설치
- `cargo binstall 패키지@버전` : 특정 버전의 패키지 설치 (기본값은 최신 버전)
- `cargo binstall -y 패키지` : 확인 프롬프트 없이 패키지 설치

## Links
- More information: https://github.com/cargo-bins/cargo-binstall
