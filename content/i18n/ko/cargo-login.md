# cargo login

레지스트리에서 로컬로 API 토큰을 저장. 토큰은 패키지 레지스트리를 인증하는 데 사용됨. `cargo logout`을 사용하여 제거할 수 있음.

## Usage
- `cargo login` : 로컬 자격 증명 저장소 (`$CARGO_HOME/credentials.toml`에 위치)에 API 토큰을 추가
- `cargo login --registry 이름` : 지정된 레지스트리를 사용 (레지스트리 이름은 구성에서 정의할 수 있음 - 기본값은 <https://crates.io>)

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-login.html
