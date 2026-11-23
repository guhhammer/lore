# bacon

Rust를 위한 백그라운드 코드 검사 도구.

## Usage
- `bacon` : 현재 디렉터리에서 변경사항이 감지될 때마다 `cargo check`를 실행
- `bacon test 경로/대상/디렉터리` : 지정한 디렉터리에서 변경 사항이 감지될 때마다 `cargo test`를 실행
- `bacon check-all` : 현재 디렉터리에서 변경 사항이 감지될 때마다 모든 타겟에 대해 `cargo check`를 실행
- `bacon run|test|clippy|doc|...` : 현재 디렉터리에서 변경 사항이 감지될 때마다 특정 작업을 실행
- `bacon --list-jobs` : 현재 사용 가능한 모든 작업을 표시
- `bacon --init` : 현재 디렉터리에서 `bacon.toml` 설정 파일을 초기화

## Links
- More information: https://github.com/Canop/bacon
