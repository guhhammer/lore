# ng update

Angular 워크스페이스 및 의존성 업데이트.

## Usage
- `ng update` : 워크스페이스의 모든 업데이트 가능한 의존성 확인 및 업데이트
- `ng update 패키지` : 특정 패키지 업데이트
- `ng update 패키지1 패키지2 ...` : 여러 패키지 업데이트
- `ng update 패키지 --force` : peer dependency 버전 충돌을 무시하고 업데이트
- `ng update 패키지 --allow-dirty` : 수정 또는 추적되지 않는 파일 포함해도 업데이트 허용
- `ng update 패키지 --next` : 베타, RC(release candidates) 등 사전 릴리스 버전으로 업데이트
- `ng update 패키지 --verbose` : 내부 동작에 대한 상세 로그 출력

## Links
- More information: https://angular.dev/cli/update
