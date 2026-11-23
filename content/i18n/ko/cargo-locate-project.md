# cargo locate-project

프로젝트의 `Cargo.toml` 매니페스트에 대한 전체 경로를 인쇄. 프로젝트가 작업공간의 일부인 경우, 작업공간의 매니페스트가 아닌 프로젝트의 매니페스트가 표시됨.

## Usage
- `cargo locate-project` : `Cargo.toml` 매니페스트에 대한 전체 경로가 포함된 JSON 객체를 표시
- `cargo locate-project --message-format plain|json` : 지정된 형식으로 프로젝트 경로를 표시
- `cargo locate-project --workspace` : 현재 작업공간 멤버가 아닌 작업공간 루트에 있는 `Cargo.toml` 매니페스트를 표시
- `cargo locate-project --manifest-path 경로/대상/Cargo.toml` : 특정 디렉터리의 `Cargo.toml` 매니페스트를 표시

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-locate-project.html
