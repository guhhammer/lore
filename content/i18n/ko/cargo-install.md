# cargo install

Rust 바이너리를 빌드하고 설치.

## Usage
- `cargo install 패키지@버전` : <https://crates.io>에서 패키지를 설치 (버전은 선택사항 - 기본적으로 최신버전)
- `cargo install --git 저장소_주소` : 지정된 Git 저장소에서 패키지를 설치
- `cargo install --git 저장소_주소 --branch|tag|rev 브랜치_이름|태그|커밋_해시` : git 저장소에서 설치할 떄 지정된 분기/태그/커밋에서 빌드
- `cargo install --path 경로/대상/패키지` : 로컬 디렉토리에서 패키지 설치
- `cargo install --list` : 설치된 모든 패키지와 해당 버전을 나열

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-install.html
