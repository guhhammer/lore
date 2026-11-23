# cargo fix

`rustc`에서 보고된 린트 경고를 자동으로 수정.

## Usage
- `cargo fix --broken-code` : 이미 컴파일러 오류가 있는 경우에도 코드를 수정
- `cargo fix --allow-dirty` : 작업 디렉터리에 변경 사항이 있어도 코드를 수정
- `cargo fix --edition` : 패키지를 다음 Rust 에디션으로 마이그래이션
- `cargo fix --lib` : 패키지 라이브러리 수정
- `cargo fix --test 이름` : 지정된 통합 테스트 수정
- `cargo fix --workspace` : 작업공간의 모든 멤버를 수정

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-fix.html
