# cargo clean

`target` 디렉터리에서 생성된 아티팩트를 제거.

## Usage
- `cargo clean` : 전체 `target` 디렉터리를 제거
- `cargo clean --doc` : 문서 아티팩트 제거 (`target/doc` 디렉터리)
- `cargo clean -r` : 릴리스 아티팩트 제거 (`target/release` 디렉터리)
- `cargo clean --profile dev` : 지정된 프로필의 디렉터리에서 아티팩트를 제거 (이 경우, `target/debug`)

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-clean.html
