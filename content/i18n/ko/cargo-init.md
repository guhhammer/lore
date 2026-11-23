# cargo init

새로운 Cargo 패키지를 생성. `cargo new`와 동일하지만, 디렉터리 지정은 선택 사항.

## Usage
- `cargo init` : 현재 디렉터리의 바이너리 대상을 사용하여 Rust 프로젝트를 초기화
- `cargo init 경로/대상/디렉터리` : 지정된 디렉터리의 바이너리 대상을 사용해 Rust 프로젝트를 초기화
- `cargo init --lib` : 현재 디렉터리의 라이브러리 대상을 사용해 Rust 프로젝트를 초기화
- `cargo init --vcs git|hg|pijul|fossil|none` : 프로젝트 디렉터리에서 버전 제어 시스템 저장소를 초기화 (기본값: `git`)
- `cargo init --name 이름` : 패키지 이름 설정 (기본값: 디렉터리 이름)

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-init.html
