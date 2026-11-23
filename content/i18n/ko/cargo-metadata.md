# cargo metadata

현재 패키지의 작업공간 멤버와 해결된 종속성을 JSON으로 출력. 참고: 출력 형식은 Cargo의 향후 버전에서 변경될 수 있음.

## Usage
- `cargo metadata` : 현재 패키지의 작업공간 멤버 및 해결된 종속성을 출력
- `cargo metadata --no-deps` : 작업공간 멤버만 출력하고 종속성을 가져오지 않음
- `cargo metadata --format-version 버전` : 지정된 버전에 따라 특정 형식으로 메타데이터를 인쇄
- `cargo metadata --filter-platform 타겟_아키텍처_정보` : 주어진 타겟 트리플에 대한 종속성만 포함하는 `resolve` 필드로 메타데이터를 출력 (참고: `packages` 배열에는 여전히 모든 대상에 대한 종속성이 포함됨)

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-metadata.html
