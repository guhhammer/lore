# cargo deb

Cargo 프로젝트로부터 Debian 패키지를 생성.

## Usage
- `cargo deb` : 프로젝트로부터 Debian 패키지 생성
- `cargo deb -o 경로/대상/파일_또는_디렉터리` : `.deb` 파일을 지정한 파일 또는 디렉토리에 저장
- `cargo deb --target x86_64-unknown-linux-gnu` : 지정한 Rust 타겟 트리플 문자열(-로 구별되는 machine-vendor-os 표현)로 컴파일
- `cargo deb -p 패키지_이름` : cargo 워크스페이스에서 사용할 패키지 선택
- `cargo deb --install` : 생성된 패키지를 즉시 설치

## Links
- More information: https://github.com/kornelski/cargo-deb
