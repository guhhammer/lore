# ctest

CMake 테스트 드라이버 프로그램.

## Usage
- `ctest -j4 --output-on-failure` : CMake 프로젝트에 정의된 모든 테스트를 실행하며 , 한번에 4개의 작업을 병렬 실행
- `ctest -N` : 사용 가능한 테스트 목록 표시
- `ctest --output-on-failure -R '^테스트명$'` : 이름을 기준으로 단일 테스트를 실행하거나 `regex` 기준으로 필터링

## Links
- More information: https://manned.org/ctest
